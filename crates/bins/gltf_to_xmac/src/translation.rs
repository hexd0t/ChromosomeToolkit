use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use formats::{
    file_formats::xmac::{
        chunks::{
            info::XmacInfo,
            material::{
                XmacLayerBlendMode, XmacMaterialLayerType, XmacMaterialTransparencyType,
                XmacStandardMaterialLayer, XmacStdMaterial,
            },
            material_info::XmacMaterialInfo,
            mesh::{XmacMesh, XmacMeshAttrib, XmacMeshAttribLayer, XmacMeshSubmesh},
            nodes::{XmacNode, XmacNodeFlags, XmacNodeId, XmacNodes},
            skinning_info::{SkinInfluence, TableEntry, XmacSkinningInfo},
            XmacChunk,
        },
        XmacFile,
    },
    types::{properties::Property, BoundingBox, Mat4, Quat, Vec2, Vec3, Vec4},
};

use gltf::{scene::Transform, Primitive, Semantic};

use crate::ConvError;

use super::Result;

#[derive(Default)]
struct TempData {
    /// from gltf node idx to xmac node idx
    node_mapping: HashMap<usize, usize>,
    /// gltf mesh idx to xmac node idx
    mesh_nodes: HashMap<usize, usize>,
}

pub fn gltf_to_xmac(
    gltf: gltf::Document,
    buffer: Vec<gltf::buffer::Data>,
    _textures: Vec<gltf::image::Data>,
    file_name: String,
    file_time: SystemTime,
) -> Result<XmacFile> {
    let mut result = XmacFile::new(file_time);

    let mut tmp = TempData::default();

    result.chunks.push(XmacChunk::Info(XmacInfo {
        unknown1: 0,
        retarget_root_node_index: -1,
        exporter_maj: 1,
        exporter_min: 0,
        unknown2: 0,
        unknown3: 0,
        source_application: gltf
            .as_json()
            .asset
            .generator
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        orig_filename: file_name,
        exporter_date: env!("CARGO_PKG_VERSION").to_string(),
        actor_name: "".to_string(),
    }));

    translate_nodes(&gltf, &mut tmp, &mut result)?;
    translate_materials(&gltf, &mut result)?;
    translate_meshes(&gltf, &buffer, &mut tmp, &mut result)?; //includes skinning

    // translate_morphs(input, &mut outputs)?;

    let (min, max) = bounding_box(result.get_mesh_chunks()[0].get_position_attrib().unwrap());
    result.res.props.push(Property {
        name: "Boundary".to_string(),
        version: 30,
        data: Box::new(formats::types::properties::PropData::BoundingBox(
            BoundingBox {
                // for some reason, min and max are reversed here.
                max: min,
                min: max,
            },
        )),
    });

    Ok(result)
}
fn bounding_box(positions: &[Vec3]) -> (Vec3, Vec3) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for pos in positions {
        let p = [pos.x, pos.y, pos.z];
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (Vec3::from_array(min), Vec3::from_array(max))
}

fn translate_nodes(gltf: &gltf::Document, tmp: &mut TempData, output: &mut XmacFile) -> Result<()> {
    let mut nodes = Vec::new();
    // maps child gltf id -> parent xmac id
    let mut parents = HashMap::new();
    let skeleton_nodes = gltf
        .skins()
        .flat_map(|s| s.joints())
        .map(|s| s.index())
        .collect::<HashSet<_>>();
    for gltf_node in gltf.nodes() {
        let gltf_node_idx = gltf_node.index();
        let name = gltf_node
            .name()
            .map(str::to_string)
            .unwrap_or_else(|| format!("Node{gltf_node_idx}"));

        // Only include nodes
        // - used for meshes
        // - used in a skelton
        // in the xmac
        if gltf_node.mesh().is_none() && !skeleton_nodes.contains(&gltf_node_idx) {
            println!("Dropping Node {gltf_node_idx} ({name})");

            continue;
        }

        let (mut local_scale, mut rotation, mut local_pos) = match gltf_node.transform() {
            Transform::Matrix { matrix } => {
                Mat4::from_cols_array_2d(&matrix).to_scale_rotation_translation()
            }
            Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => (
                Vec3::from_array(scale),
                Quat::from_array(rotation),
                Vec3::from_array(translation),
            ),
        };

        // R1 canonicalizes by always keeping w component of rotation positive
        if rotation.w.is_sign_negative() {
            rotation.x *= -1.0;
            rotation.y *= -1.0;
            rotation.z *= -1.0;
            rotation.w *= -1.0;
        }

        local_scale = local_scale.recip();
        local_pos *= 100.0; //m to cm
        let parent_idx = parents.get(&gltf_node_idx).copied();

        let node = XmacNode {
            name,
            rotation,
            unknown1: Vec4::W,
            local_pos,
            local_scale,
            unknown2: Vec3::NEG_ONE,
            unknown3: -1,
            unknown4: -1,
            parent_idx,
            child_count: gltf_node.children().len() as u32,
            flags: XmacNodeFlags::IncludeInBoundsCalc,
            unknown5: [0, 0, 0],
            oriented_bounding_box: Mat4::IDENTITY, // TODO: Calculate actual bounding box
            unknown6: 1.0,
        };
        let xmac_node_id = nodes.len();
        tmp.node_mapping.insert(gltf_node_idx, xmac_node_id);
        nodes.push(node);
        for child_id in gltf_node.children() {
            parents.insert(child_id.index(), xmac_node_id);
        }
        if let Some(mesh) = gltf_node.mesh() {
            tmp.mesh_nodes.insert(mesh.index(), xmac_node_id);
        }
    }
    let nodes_chunk = XmacNodes { nodes };
    output.chunks.push(XmacChunk::Nodes(nodes_chunk));

    Ok(())
}

type TempSkinData = Vec<[(u32, f32); 8]>;
#[derive(Default)]
struct VertexData {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    tangents: Vec<Vec4>,
    uvs: Vec<Vec2>,
    /// still as gltf joint idx
    joints: Vec<[u32; 4]>,
    weights: Vec<[f32; 4]>,
    joints2: Vec<[u32; 4]>,
    weights2: Vec<[f32; 4]>,
}

fn translate_meshes(
    gltf: &gltf::Document,
    buffer: &[gltf::buffer::Data],
    tmp: &mut TempData,
    output: &mut XmacFile,
) -> Result<()> {
    for gltf_mesh in gltf.meshes() {
        let gltf_mesh_idx = gltf_mesh.index();
        let gltf_node = gltf
            .nodes()
            .find(|n| n.mesh().is_some_and(|m| m.index() == gltf_mesh_idx))
            .unwrap();
        let mesh_node_idx = *tmp.mesh_nodes.get(&gltf_mesh_idx).unwrap();
        let mesh_node = &output.get_nodes_chunk().unwrap().nodes[mesh_node_idx];

        let mut submeshes = Vec::with_capacity(gltf_mesh.primitives().len());
        let mut vertices = VertexData::default();

        // If multiple submeshes point to the same vertex buffer, their indices will keep counting
        // We need indices to be in range 0..submesh_vertex_count though per submesh
        let mut vertices_in_buffer_used = HashMap::<usize, u32>::new();
        let mut prev_used_buffer = None;

        for prim in gltf_mesh.primitives() {
            if prim.mode() != gltf::json::mesh::Mode::Triangles {
                return Err(ConvError::NotImplemented(format!(
                    "Only Triangle meshes are supported, found {:?}",
                    prim.mode()
                )));
            }

            let prim_start_vertex = vertices.positions.len();
            let read = prim.reader(|buf| buffer.get(buf.index()).map(|v| v.0.as_slice()));

            let prim_pos_buffer = prim
                .attributes()
                .find(|a| a.0 == Semantic::Positions)
                .unwrap()
                .1;
            let previous_verts = vertices_in_buffer_used
                .entry(prim_pos_buffer.index())
                .or_insert(0);
            if *previous_verts == 0 {
                // These vertices have not yet been read
                read_prim_vertex_data(&gltf_node, &prim, &read, &mut vertices, tmp)?;
            } else if prev_used_buffer != Some(prim_pos_buffer.index()) {
                return Err(ConvError::NotImplemented(
                    "Vertex Buffer reuses must be consecutive".to_string(),
                ));
            }
            prev_used_buffer = Some(prim_pos_buffer.index());

            // actually one-past the last idx:
            let prim_end_vertex = vertices.positions.len();
            let prim_new_vertices = prim_end_vertex - prim_start_vertex;

            let indices: Vec<u32> = if let Some(indices) = read.read_indices() {
                if *previous_verts == 0 {
                    indices.into_u32().collect()
                } else {
                    indices.into_u32().map(|i| i - *previous_verts).collect()
                }
            } else {
                assert_ne!(
                    prim_new_vertices, 0,
                    "Encountered Prim with reused buffer and no indices!"
                );
                (0..prim_new_vertices as u32).collect()
            };
            let prim_vertices = *indices.iter().max().unwrap() + 1;
            if prim_vertices + *previous_verts > prim_end_vertex as u32 {
                return Err(ConvError::InvalidData(format!(
                    "Indices in submesh {} exceed vertex count \
                    (range {} + {prim_vertices} of {prim_end_vertex})",
                    prim.index(),
                    *previous_verts
                )));
            }

            filter_weightless_joints(&mut vertices, prim_start_vertex);

            let joints1 = indices
                .iter()
                .flat_map(|i| vertices.joints[(*i + *previous_verts) as usize].iter());
            let joints2 = indices.iter().flat_map(|i| {
                vertices
                    .joints2
                    .get((*i + *previous_verts) as usize)
                    .unwrap_or(&[u32::MAX; 4])
                    .iter()
            });
            let mut bones: Vec<_> = joints1
                .chain(joints2)
                .copied()
                .filter(|j| *j < u32::MAX)
                .collect();
            bones.sort();
            bones.dedup();

            *previous_verts += prim_vertices;

            let submesh = XmacMeshSubmesh {
                indices,
                bones,
                vertices_count: prim_vertices,
                material_idx: prim
                    .material()
                    .index()
                    .expect("Implicit materials are not supported")
                    as u32,
            };
            submeshes.push(submesh);
        }

        let orig_verts_count = vertices.positions.len() as u32;

        let mut vertex_attribute_layers = Vec::new();
        let mut weight_data = None;
        mesh_vertices_to_attrib_layers(vertices, &mut vertex_attribute_layers, &mut weight_data);

        let is_collision_mesh = mesh_node.name.to_lowercase().contains("collision");

        let mesh = XmacMesh {
            vertex_attribute_layers,
            submeshes,
            node_id: XmacNodeId(mesh_node_idx as u32),
            orig_verts_count,
            is_collision_mesh,
            unknown1: [0; 3],
        };
        output.chunks.push(XmacChunk::Mesh(mesh));

        if let Some(skin) = weight_data {
            translate_skinning(output, mesh_node_idx, is_collision_mesh, skin);
        }
    }

    Ok(())
}

fn translate_skinning(
    output: &mut XmacFile,
    mesh_node_idx: usize,
    is_collision_mesh: bool,
    skin: TempSkinData,
) {
    let mut table_entries = Vec::new();
    let mut influences = Vec::new();
    let mut local_bones = HashSet::new();

    for vertex in skin {
        let mut num_elements = 0;
        let start_idx = influences.len() as u32;
        for (bone_idx, weight) in vertex {
            if bone_idx == u32::MAX {
                continue;
            }
            local_bones.insert(bone_idx);
            influences.push(SkinInfluence {
                weight,
                node_idx: bone_idx as u16,
                unknown: 0,
            });
            num_elements += 1;
        }
        table_entries.push(TableEntry {
            start_idx,
            num_elements,
        });
    }

    let skin = XmacSkinningInfo {
        node_id: XmacNodeId(mesh_node_idx as u32),
        influences,
        table_entries,
        local_bones: local_bones.len() as u32,
        is_for_collision_mesh: is_collision_mesh,
        unknown1: [0, 0, 0],
    };
    output.chunks.push(XmacChunk::SkinningInfo(skin));
}

fn filter_weightless_joints(vertices: &mut VertexData, prim_start_vertex: usize) {
    // set joint to u16::MAX (invalid) if weight ~= 0
    for (joint, weight) in vertices
        .joints
        .iter_mut()
        .flatten()
        .zip(vertices.weights.iter().flatten())
        .skip(prim_start_vertex * 4)
        .chain(
            vertices
                .joints2
                .iter_mut()
                .flatten()
                .zip(vertices.weights2.iter().flatten())
                .skip(prim_start_vertex * 4),
        )
    {
        if *weight <= f32::EPSILON && *weight >= -f32::EPSILON {
            *joint = u32::MAX;
        }
    }
}

fn mesh_vertices_to_attrib_layers(
    vertices: VertexData,
    vertex_attribute_layers: &mut Vec<XmacMeshAttribLayer>,
    weight_data: &mut Option<TempSkinData>,
) {
    let VertexData {
        positions,
        normals,
        tangents,
        uvs,
        joints,
        weights,
        joints2,
        weights2,
    } = vertices;

    vertex_attribute_layers.push(XmacMeshAttribLayer {
        attribs: XmacMeshAttrib::OriginalVertexNumbers((0..positions.len() as u32).collect()),
        flag1: false,
        unknown1: [0, 0, 0],
    });
    vertex_attribute_layers.push(XmacMeshAttribLayer {
        attribs: XmacMeshAttrib::Positions(positions),
        flag1: true,
        unknown1: [0, 0, 0],
    });

    if !normals.is_empty() {
        vertex_attribute_layers.push(XmacMeshAttribLayer {
            attribs: XmacMeshAttrib::Normals(normals),
            flag1: true,
            unknown1: [0, 0, 0],
        });
    }
    if !tangents.is_empty() {
        vertex_attribute_layers.push(XmacMeshAttribLayer {
            attribs: XmacMeshAttrib::Tangents(tangents),
            flag1: true,
            unknown1: [0, 0, 0],
        });
    }
    if !uvs.is_empty() {
        vertex_attribute_layers.push(XmacMeshAttribLayer {
            attribs: XmacMeshAttrib::UvCoords(uvs),
            flag1: false,
            unknown1: [0, 0, 0],
        });
    }
    let joints = joints.into_iter();
    let mut joints2 = joints2.into_iter();
    let mut weights = weights.into_iter();
    let mut weights2 = weights2.into_iter();
    let mut temp_skin = Vec::new();
    for joint in joints {
        let joint2 = joints2.next().unwrap_or([u32::MAX; 4]);
        let weight = weights.next().unwrap();
        let weight2 = weights2.next().unwrap_or_default();

        temp_skin.push([
            (joint[0], weight[0]),
            (joint[1], weight[1]),
            (joint[2], weight[2]),
            (joint[3], weight[3]),
            (joint2[0], weight2[0]),
            (joint2[1], weight2[1]),
            (joint2[2], weight2[2]),
            (joint2[3], weight2[3]),
        ]);
    }
    *weight_data = Some(temp_skin);
}

fn read_prim_vertex_data<'a, 's, T: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>>(
    gltf_node: &gltf::Node,
    prim: &Primitive,
    read: &gltf::mesh::Reader<'a, 's, T>,
    vertices: &mut VertexData,
    tmp: &mut TempData,
) -> Result<()> {
    let gltf_skin = gltf_node.skin();

    let joint_idx_to_new_bone_id = |joint: u16| -> u32 {
        tmp.node_mapping
            .get(
                &gltf_skin
                    .as_ref()
                    .expect("Found skinning weights, but no Skin")
                    .joints()
                    .nth(joint as usize)
                    .unwrap()
                    .index(),
            )
            .copied()
            .map(|v| v as u32)
            .unwrap_or(u32::MAX)
    };
    let translate_bone_ids = |gltf_id: [u16; 4]| {
        [
            joint_idx_to_new_bone_id(gltf_id[0]),
            joint_idx_to_new_bone_id(gltf_id[1]),
            joint_idx_to_new_bone_id(gltf_id[2]),
            joint_idx_to_new_bone_id(gltf_id[3]),
        ]
    };

    for (attr_type, _) in prim.attributes() {
        match attr_type {
            gltf::Semantic::Positions => vertices.positions.extend(
                read.read_positions()
                    .unwrap()
                    .map(Vec3::from_array)
                    .map(|p| p * 100.0),
            ), //m to cm
            gltf::Semantic::Normals => vertices
                .normals
                .extend(read.read_normals().unwrap().map(Vec3::from_array)),
            gltf::Semantic::Tangents => vertices
                .tangents
                .extend(read.read_tangents().unwrap().map(Vec4::from_array)),
            gltf::Semantic::TexCoords(0) => vertices.uvs.extend(
                read.read_tex_coords(0)
                    .unwrap()
                    .into_f32()
                    .map(Vec2::from_array),
            ),
            gltf::Semantic::Joints(0) => vertices.joints.extend(
                read.read_joints(0)
                    .unwrap()
                    .into_u16()
                    .map(translate_bone_ids),
            ),
            gltf::Semantic::Weights(0) => vertices
                .weights
                .extend(read.read_weights(0).unwrap().into_f32()),
            gltf::Semantic::Joints(1) => vertices.joints2.extend(
                read.read_joints(1)
                    .unwrap()
                    .into_u16()
                    .map(translate_bone_ids),
            ),
            gltf::Semantic::Weights(1) => vertices
                .weights2
                .extend(read.read_weights(1).unwrap().into_f32()),
            _ => {
                println!("{attr_type:?} is not implemented, ignoring")
            }
        }
    }
    if !vertices.normals.is_empty() && vertices.normals.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} normals but {} positions!",
            prim.index(),
            vertices.normals.len(),
            vertices.positions.len()
        )));
    }
    if !vertices.tangents.is_empty() && vertices.tangents.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} tangents but {} positions!",
            prim.index(),
            vertices.tangents.len(),
            vertices.positions.len()
        )));
    }
    if !vertices.uvs.is_empty() && vertices.uvs.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} uvs but {} positions!",
            prim.index(),
            vertices.uvs.len(),
            vertices.positions.len()
        )));
    }
    if !vertices.joints.is_empty() && vertices.joints.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} joints but {} positions!",
            prim.index(),
            vertices.joints.len(),
            vertices.positions.len()
        )));
    }
    if !vertices.weights.is_empty() && vertices.weights.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} weights but {} positions!",
            prim.index(),
            vertices.weights.len(),
            vertices.positions.len()
        )));
    }
    if !vertices.joints2.is_empty() && vertices.joints2.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} joints2 but {} positions!",
            prim.index(),
            vertices.joints2.len(),
            vertices.positions.len()
        )));
    }
    if !vertices.weights2.is_empty() && vertices.weights2.len() != vertices.positions.len() {
        return Err(ConvError::InvalidData(format!(
            "Inhomogeneous Vertex Data between primitives is not supported, \
            after reading {}, found {} weights2 but {} positions!",
            prim.index(),
            vertices.weights2.len(),
            vertices.positions.len()
        )));
    }
    Ok(())
}

fn translate_materials(gltf: &gltf::Document, output: &mut XmacFile) -> Result<()> {
    output
        .chunks
        .push(XmacChunk::MaterialInfo(XmacMaterialInfo::new(
            gltf.materials().len(),
        )));

    for gltf_material in gltf.materials() {
        let material_idx = gltf_material
            .index()
            .expect("Implicit material encountered") as u16;
        let name = gltf_material
            .name()
            .map(str::to_string)
            .unwrap_or_else(|| format!("Mat{material_idx}",));
        let mut layers = Vec::new();

        let blend_mode = match gltf_material.alpha_mode() {
            gltf::material::AlphaMode::Opaque => XmacLayerBlendMode::None,
            gltf::material::AlphaMode::Mask | gltf::material::AlphaMode::Blend => {
                XmacLayerBlendMode::Over
            }
        };

        if let Some(gltf_diffuse) = gltf_material.pbr_metallic_roughness().base_color_texture() {
            layers.push(gltf_texture_info_to_xmac_layer(
                XmacMaterialLayerType::Diffuse,
                material_idx,
                blend_mode,
                &gltf_diffuse,
            ));
        }
        if let Some(gltf_normal) = gltf_material.normal_texture() {
            layers.push(gltf_normal_to_xmac_layer(
                XmacMaterialLayerType::Bump,
                material_idx,
                blend_mode,
                &gltf_normal,
            ));
        }

        let shine_strength = if let Some(gltf_specular) = gltf_material.specular() {
            if let Some(gltf_spec_tex) = gltf_specular.specular_texture() {
                layers.push(gltf_texture_info_to_xmac_layer(
                    XmacMaterialLayerType::Specular,
                    material_idx,
                    blend_mode,
                    &gltf_spec_tex,
                ));
            }
            gltf_specular.specular_factor()
        } else {
            0.0
        };
        let unused_color = Vec4::new(0.0, 0.0, 0.0, f32::MIN_POSITIVE);
        let opacity = gltf_material.pbr_metallic_roughness().base_color_factor()[3];
        let refraction_index = gltf_material.ior().unwrap_or(1.0);

        let mat = XmacStdMaterial {
            name,
            layers,
            ambient_color: unused_color,
            diffuse_color: unused_color,
            specular_color: unused_color,
            emissive_color: unused_color,
            shine: 25.0,
            shine_strength,
            opacity,
            refraction_index,
            double_sided: gltf_material.double_sided(),
            wireframe: false,
            transparency_type: XmacMaterialTransparencyType::Filter,
        };
        output.chunks.push(XmacChunk::StdMaterial(mat));
    }
    Ok(())
}

fn gltf_texture_to_xmac_layer(
    ty: XmacMaterialLayerType,
    material_id: u16,
    blend_mode: XmacLayerBlendMode,
    input: &gltf::texture::Texture,
) -> XmacStandardMaterialLayer {
    let tex_name_regex =
        regex::Regex::new(r"(?:.*/)?([^/]+?)(?:\.[pP][nN][gG])?(?:\.[jJ][pP][eE]?[gG])?$").unwrap();
    let texture = match input.source().source() {
        // if existing, prefer to use texture filename without extension:
        gltf::image::Source::Uri { uri, mime_type: _ } => tex_name_regex
            .captures(uri)
            .expect("Unexpected filename")
            .get(1)
            .unwrap()
            .as_str()
            .to_string(),
        gltf::image::Source::View {
            view: _,
            mime_type: _,
        } => {
            if let Some(tex_name) = input.name() {
                tex_name.to_string()
            } else if let Some(img_name) = input.source().name() {
                img_name.to_string()
            } else {
                panic!("No source for texture name");
            }
        }
    };
    XmacStandardMaterialLayer {
        ty,
        texture,
        amount: 1.0,
        u_offset: 0.0,
        v_offset: 0.0,
        u_tiling: 1.0,
        v_tiling: 1.0,
        rotation_rads: 0.0,
        material_id,
        blend_mode,
    }
}

fn gltf_texture_info_to_xmac_layer(
    ty: XmacMaterialLayerType,
    material_id: u16,
    blend_mode: XmacLayerBlendMode,
    input: &gltf::texture::Info,
) -> XmacStandardMaterialLayer {
    let mut result = gltf_texture_to_xmac_layer(ty, material_id, blend_mode, &input.texture());

    if let Some(tex_trans) = input.texture_transform() {
        result.u_offset = tex_trans.offset()[0];
        result.v_offset = tex_trans.offset()[1];
        result.u_tiling = tex_trans.scale()[0];
        result.v_tiling = tex_trans.scale()[1];
        result.rotation_rads = tex_trans.rotation();
    }

    result
}

fn gltf_normal_to_xmac_layer(
    ty: XmacMaterialLayerType,
    material_id: u16,
    blend_mode: XmacLayerBlendMode,
    input: &gltf::material::NormalTexture,
) -> XmacStandardMaterialLayer {
    let mut result = gltf_texture_to_xmac_layer(ty, material_id, blend_mode, &input.texture());

    result.u_tiling = input.scale();
    result.v_tiling = input.scale();

    result
}
