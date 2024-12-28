use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsString,
    io::{BufWriter, Write},
    path::Path,
};

use super::{ConvError, Result};
use formats::{
    helpers::{write_f32, write_u16},
    types::{Vector2, Vector3, Vector4},
    xmac::{
        chunks::{
            material::{XmacLayerBlendMode, XmacMaterialLayerType, XmacStandardMaterialLayer},
            mesh::{XmacMesh, XmacMeshAttrib, XmacMeshAttribLayer, XmacMeshSubmesh},
            skinning_info::XmacSkinningInfo,
            XmacChunk,
        },
        XmacFile,
    },
};
use gltf::json::{
    extensions::material::{IndexOfRefraction, Ior, Material, Specular, SpecularFactor},
    image::MimeType,
    material::{EmissiveFactor, NormalTexture, PbrMetallicRoughness, StrengthFactor},
    mesh::Primitive as GltfPrimitive,
    texture::Info,
    validation::Checked::Valid as GltfValid,
    Accessor as GltfAccessor, Buffer as GltfBuffer, Image, Index as GltfIndex,
    Material as GltfMaterial, Mesh as GltfMesh, Node as GltfNode, Root as GltfRoot,
    Skin as GltfSkin, Texture,
};

pub fn xmac_to_gltf(input: &XmacFile, src_filepath: &str) -> Result<GltfRoot> {
    let mut output = GltfRoot::default();

    output.extensions_used.push("KHR_materials_ior".to_string());
    output
        .extensions_used
        .push("KHR_materials_specular".to_string());

    translate_nodes(input, &mut output)?;
    translate_materials(input, &mut output)?;
    translate_meshes(input, &mut output, src_filepath)?;
    translate_skinning(input, &mut output, src_filepath)?;

    Ok(output)
}

fn translate_nodes(input: &XmacFile, output: &mut GltfRoot) -> Result<()> {
    if let Some(nodes) = input.get_nodes_chunk() {
        for (node_idx, node) in nodes.into_iter().enumerate() {
            let mut scale = node.local_scale.clone();
            //for some reason, scale is inverted (maybe only when multiply-order == 1?):
            scale.x = 1.0 / scale.x;
            scale.y = 1.0 / scale.y;
            scale.z = 1.0 / scale.z;

            let gltf_node = GltfNode {
                name: Some(node.name.clone()),
                rotation: Some(node.rotation.clone().into()),
                translation: Some(node.local_pos.clone().into()),
                scale: Some(scale.into()),

                ..GltfNode::default()
            };
            output.nodes.push(gltf_node);
            if let Some(parent_id) = node.parent_idx {
                if parent_id >= output.nodes.len() {
                    return Err(ConvError::NotImplemented(format!(
                        "Node {node_idx} encountered before its parent {parent_id}"
                    )));
                }
                let parent_children = output.nodes[parent_id].children.get_or_insert_default();
                parent_children.push(GltfIndex::new(node_idx as u32));
            }
        }
    } else {
        return Err(ConvError::MandatoryDataMissing(
            "Nodes chunk is missing!".to_string(),
        ));
    }
    Ok(())
}

fn translate_meshes(input: &XmacFile, output: &mut GltfRoot, src_filepath: &str) -> Result<()> {
    for mesh in input.get_mesh_chunks() {
        let gltf_node = &mut output.nodes[mesh.node_id.0 as usize];
        let mesh_idx = output.meshes.len();
        if gltf_node.mesh.is_some() {
            return Err(ConvError::InvalidData(format!(
                "Node {} is referenced by multiple meshes!",
                mesh.node_id.0
            )));
        }
        gltf_node.mesh = Some(GltfIndex::new(mesh_idx as u32));

        let mesh_name = gltf_node
            .name
            .as_ref()
            .map(|node_name| format!("{node_name}_mesh",))
            .unwrap_or_else(|| format!("mesh_{}", mesh.node_id.0));
        let mesh_buffer_path =
            OsString::from(src_filepath.replace("._xmac", &format!("._xmac.{}.bin", &mesh_name)));
        let (positions, normals, tangents, uvs) =
            create_mesh_buffer(mesh, output, &Path::new(&mesh_buffer_path))?;

        let mut primitives = Vec::with_capacity(mesh.submeshes.len());
        let mut index_offset = 0;
        for (idx, submesh) in mesh.submeshes.iter().enumerate() {
            let submesh_buffer_path = OsString::from(
                src_filepath.replace("._xmac", &format!("._xmac.{}_sub{idx}.bin", &mesh_name)),
            );
            let indices = create_submesh_buffer(
                submesh,
                output,
                &Path::new(&submesh_buffer_path),
                index_offset,
            )?;
            let primitive = GltfPrimitive {
                attributes: {
                    let mut map = BTreeMap::new();
                    map.insert(GltfValid(gltf::json::mesh::Semantic::Positions), positions);
                    map.insert(GltfValid(gltf::json::mesh::Semantic::Normals), normals);
                    map.insert(GltfValid(gltf::json::mesh::Semantic::Tangents), tangents);
                    map.insert(GltfValid(gltf::json::mesh::Semantic::TexCoords(0)), uvs);
                    map
                },
                extensions: None,
                extras: None,
                indices: Some(indices),
                material: Some(GltfIndex::new(submesh.material_idx)), //Materials are exported 1:1, so their idx is stable
                mode: GltfValid(gltf::mesh::Mode::Triangles),
                targets: None,
            };
            primitives.push(primitive);
            index_offset += submesh.vertices_count;
        }

        let gltf_mesh = GltfMesh {
            extensions: None,
            extras: None,
            name: Some(mesh_name),
            primitives,
            weights: None,
        };

        output.meshes.push(gltf_mesh);
    }
    Ok(())
}

fn bounding_box(positions: &[Vector3]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for pos in positions {
        let p = [pos.x, pos.y, pos.z];
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}

fn create_mesh_buffer(
    mesh: &XmacMesh,
    output: &mut GltfRoot,
    buffer_path: &Path,
) -> Result<(
    GltfIndex<GltfAccessor>,
    GltfIndex<GltfAccessor>,
    GltfIndex<GltfAccessor>,
    GltfIndex<GltfAccessor>,
)> {
    use gltf::json::validation::USize64;

    fn get_position_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vector3>> {
        if let XmacMeshAttrib::Positions(val) = &attrib.attribs {
            Some(val)
        } else {
            None
        }
    }
    fn get_normal_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vector3>> {
        if let XmacMeshAttrib::Normals(val) = &attrib.attribs {
            Some(val)
        } else {
            None
        }
    }
    fn get_tangent_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vector4>> {
        if let XmacMeshAttrib::Tangents(val) = &attrib.attribs {
            Some(val)
        } else {
            None
        }
    }
    fn get_uv_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vector2>> {
        if let XmacMeshAttrib::UvCoords(val) = &attrib.attribs {
            Some(val)
        } else {
            None
        }
    }

    let mesh_positions = mesh
        .vertex_attribute_layers
        .iter()
        .find_map(get_position_attrib)
        .unwrap();
    let mesh_normals = mesh
        .vertex_attribute_layers
        .iter()
        .find_map(get_normal_attrib)
        .unwrap();
    let mesh_tangents = mesh
        .vertex_attribute_layers
        .iter()
        .find_map(get_tangent_attrib)
        .unwrap();
    let mesh_uvs = mesh
        .vertex_attribute_layers
        .iter()
        .find_map(get_uv_attrib)
        .unwrap();

    if mesh.vertex_attribute_layers.len() != 5 {
        //4 used here + orig_vert_idx in skinning
        println!("Warning: xmac has additional mesh attrib layers which will not be exported!");
    }

    const POSITION_SIZE: usize = std::mem::size_of::<Vector3>();
    const NORMAL_SIZE: usize = std::mem::size_of::<Vector3>();
    const TANGENT_SIZE: usize = std::mem::size_of::<Vector4>();
    const UV_SIZE: usize = std::mem::size_of::<Vector2>();
    const VERTEX_SIZE: usize = POSITION_SIZE + NORMAL_SIZE + TANGENT_SIZE + UV_SIZE;

    let vertex_count = mesh_positions.len();
    let bounding = bounding_box(&mesh_positions);

    let buffer_length = vertex_count * VERTEX_SIZE;
    let buffer = output.push(GltfBuffer {
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: Some(
            buffer_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        ),
    });
    let buffer_view = output.push(gltf::json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: Some(gltf::json::buffer::Stride(VERTEX_SIZE)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(GltfValid(gltf::json::buffer::Target::ArrayBuffer)),
    });
    let gltf_positions = output.push(GltfAccessor {
        buffer_view: Some(buffer_view.clone()),
        byte_offset: Some(USize64(0)),
        count: USize64::from(vertex_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Vec3),
        min: Some(gltf::json::Value::from(Vec::from(bounding.0))),
        max: Some(gltf::json::Value::from(Vec::from(bounding.1))),
        name: None,
        normalized: false,
        sparse: None,
    });
    let gltf_normals = output.push(GltfAccessor {
        buffer_view: Some(buffer_view.clone()),
        byte_offset: Some(USize64(POSITION_SIZE as u64)),
        count: USize64::from(vertex_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    let gltf_tangents = output.push(GltfAccessor {
        buffer_view: Some(buffer_view.clone()),
        byte_offset: Some(USize64((POSITION_SIZE + NORMAL_SIZE) as u64)),
        count: USize64::from(vertex_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Vec4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    let gltf_uv = output.push(GltfAccessor {
        buffer_view: Some(buffer_view.clone()),
        byte_offset: Some(USize64((POSITION_SIZE + NORMAL_SIZE + TANGENT_SIZE) as u64)),
        count: USize64::from(vertex_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Vec2),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let buffer_file = std::fs::File::create(buffer_path)?;
    let mut buffer_file = BufWriter::new(buffer_file);

    for idx in 0..vertex_count {
        mesh_positions[idx].save(&mut buffer_file)?;
        mesh_normals[idx].save(&mut buffer_file)?;
        mesh_tangents[idx].save(&mut buffer_file)?;
        mesh_uvs[idx].save(&mut buffer_file)?;
    }
    buffer_file.flush()?;

    Ok((gltf_positions, gltf_normals, gltf_tangents, gltf_uv))
}

fn create_submesh_buffer(
    submesh: &XmacMeshSubmesh,
    output: &mut GltfRoot,
    buffer_path: &Path,
    index_offset: u32,
) -> Result<GltfIndex<GltfAccessor>> {
    use gltf::json::validation::USize64;

    const INDEX_SIZE: usize = std::mem::size_of::<u32>();

    let buffer_length = submesh.indices.len() * INDEX_SIZE;
    let buffer = output.push(GltfBuffer {
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: Some(
            buffer_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        ),
    });
    let buffer_view = output.push(gltf::json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: None, //Non-Vertexbuffers must not have stride
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(GltfValid(gltf::json::buffer::Target::ElementArrayBuffer)),
    });
    let gltf_indices = output.push(GltfAccessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(submesh.indices.len()),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::U32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let buffer_file = std::fs::File::create(buffer_path)?;
    let mut buffer_file = BufWriter::new(buffer_file);

    for index in &submesh.indices {
        formats::helpers::write_u32(&mut buffer_file, *index + index_offset)?;
    }
    buffer_file.flush()?;

    Ok(gltf_indices)
}

fn translate_skinning<'a>(
    input: &'a XmacFile,
    output: &mut GltfRoot,
    src_filepath: &str,
) -> Result<()> {
    fn get_skinning_chunk(chunk: &XmacChunk) -> Option<&XmacSkinningInfo> {
        if let XmacChunk::SkinningInfo(skin) = chunk {
            Some(skin)
        } else {
            None
        }
    }
    for skin in input.chunks.iter().filter_map(get_skinning_chunk) {
        let node_id = skin.node_id;
        let skin_mesh = input.get_mesh_chunk(node_id).ok_or_else(|| {
            ConvError::MandatoryDataMissing(format!(
                "Skinning Info for {node_id:?} is missing Mesh data!"
            ))
        })?;
        let gltf_node = &output.nodes[node_id.0 as usize];
        let skin_name = gltf_node
            .name
            .as_ref()
            .map(|node_name| format!("{node_name}_skin",))
            .unwrap_or_else(|| format!("skin_{}", node_id.0));

        let (node_to_joint_idx, joints) = calculate_relevant_joint_ids(input, skin);

        let gltf_skin = GltfSkin {
            extensions: None,
            extras: None,
            inverse_bind_matrices: None,
            joints,
            name: None,
            skeleton: None,
        };
        let gltf_skin = output.push(gltf_skin);
        let gltf_node = &mut output.nodes[node_id.0 as usize];
        gltf_node.skin = Some(gltf_skin);

        let mesh_buffer_path =
            OsString::from(src_filepath.replace("._xmac", &format!("._xmac.{}.bin", &skin_name)));
        let (gltf_joints, gltf_weights) = create_skin_buffer(
            skin,
            skin_mesh,
            node_to_joint_idx,
            output,
            &Path::new(&mesh_buffer_path),
        )?;

        let gltf_mesh = output
            .meshes
            .get_mut(output.nodes[node_id.0 as usize].mesh.unwrap().value())
            .unwrap();
        for prim in gltf_mesh.primitives.iter_mut() {
            prim.attributes.insert(
                GltfValid(gltf::json::mesh::Semantic::Joints(0)),
                gltf_joints.clone(),
            );
            prim.attributes.insert(
                GltfValid(gltf::json::mesh::Semantic::Weights(0)),
                gltf_weights.clone(),
            );
        }
    }
    Ok(())
}

fn calculate_relevant_joint_ids(
    input: &XmacFile,
    skin: &XmacSkinningInfo,
) -> (HashMap<u16, u16>, Vec<GltfIndex<GltfNode>>) {
    // First, look at all joint node_idxs referenced in the skin, sort and deduplicate them:
    let mut joints = skin
        .influences
        .iter()
        .map(|inf| inf.node_idx)
        .collect::<Vec<_>>();
    joints.sort();
    joints.dedup();

    // There might be intermediate joints which aren't referred to by geometry,
    // but are still part of this skeleton:
    let nodes = input.get_nodes_chunk().unwrap();
    let mut idx = 0;
    // for all joints:
    while idx < joints.len() {
        let mut joint_node_idx = joints[idx];
        // if we find missing joints, follow the chain until the parent is part of the list again
        // (or it is a root node):
        loop {
            let joint_node = &nodes.nodes[joint_node_idx as usize];
            if let Some(parent_id) = joint_node.parent_idx {
                let parent_id = parent_id as u16;
                if let Err(insert_loc) = joints.binary_search(&parent_id) {
                    joint_node_idx = parent_id;
                    joints.insert(insert_loc, joint_node_idx);
                    idx += 1;
                } else {
                    break; // parent is part of list already
                }
            } else {
                break; // is root
            }
        }
        idx += 1;
    }

    let node_to_joint_idx: HashMap<u16, u16> = joints
        .iter()
        .enumerate()
        .map(|(joint_idx, node_idx)| (*node_idx, joint_idx as u16))
        .collect();
    let joints = joints
        .into_iter()
        .map(|bone_node_id| GltfIndex::new(bone_node_id as u32))
        .collect::<Vec<_>>();
    (node_to_joint_idx, joints)
}

fn create_skin_buffer(
    skin: &XmacSkinningInfo,
    skin_mesh: &XmacMesh,
    node_to_joint_idx: HashMap<u16, u16>,
    output: &mut GltfRoot,
    buffer_path: &Path,
) -> Result<(GltfIndex<GltfAccessor>, GltfIndex<GltfAccessor>)> {
    use gltf::json::validation::USize64;
    fn get_orig_vert_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<u32>> {
        if let XmacMeshAttrib::OriginalVertexNumbers(val) = &attrib.attribs {
            Some(val)
        } else {
            None
        }
    }

    let vertex_to_orig_vertex = skin_mesh
        .vertex_attribute_layers
        .iter()
        .find_map(get_orig_vert_attrib)
        .unwrap();

    // glTf skinning is usually restricted to 4 influences:
    const JOINT_IDX_SIZE: usize = std::mem::size_of::<u16>() * 4;
    const JOINT_WEIGHT_SIZE: usize = std::mem::size_of::<f32>() * 4;
    const SKIN_ENTRY_SIZE: usize = JOINT_IDX_SIZE + JOINT_WEIGHT_SIZE;

    let vertex_count = vertex_to_orig_vertex.len();

    let buffer_length = vertex_count * SKIN_ENTRY_SIZE;
    let buffer = output.push(GltfBuffer {
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: Some(
            buffer_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        ),
    });
    let buffer_view = output.push(gltf::json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: Some(gltf::json::buffer::Stride(SKIN_ENTRY_SIZE)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(GltfValid(gltf::json::buffer::Target::ArrayBuffer)),
    });
    let gltf_joints = output.push(GltfAccessor {
        buffer_view: Some(buffer_view.clone()),
        byte_offset: Some(USize64(0)),
        count: USize64::from(vertex_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::U16,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Vec4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    let gltf_weights = output.push(GltfAccessor {
        buffer_view: Some(buffer_view.clone()),
        byte_offset: Some(USize64(JOINT_IDX_SIZE as u64)),
        count: USize64::from(vertex_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Vec4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let buffer_file = std::fs::File::create(buffer_path)?;
    let mut buffer_file = BufWriter::new(buffer_file);

    for (_vertex, orig_vertex) in vertex_to_orig_vertex.iter().enumerate() {
        let lookup = &skin.table_entries[*orig_vertex as usize];

        let mut joints = Vec::with_capacity(4);
        let mut weights = Vec::with_capacity(4);
        assert!(lookup.num_elements <= 4);
        for lookup_offset in 0..lookup.num_elements {
            let influence = &skin.influences[(lookup.start_idx + lookup_offset) as usize];
            joints.push(*node_to_joint_idx.get(&influence.node_idx).unwrap());
            weights.push(influence.weight);
        }
        for _free_slot in joints.len()..4 {
            joints.push(0);
            weights.push(0.0);
        }
        for joint in joints.into_iter() {
            write_u16(&mut buffer_file, joint)?;
        }
        for weight in weights.into_iter() {
            write_f32(&mut buffer_file, weight)?;
        }
    }

    buffer_file.flush()?;

    Ok((gltf_joints, gltf_weights))
}

fn translate_materials(input: &XmacFile, output: &mut GltfRoot) -> Result<()> {
    for material in input.get_material_chunks() {
        let diffuse_layer = material
            .get_layer_by_type(XmacMaterialLayerType::Diffuse)
            .unwrap();
        let specular_layer = material.get_layer_by_type(XmacMaterialLayerType::Bump);
        let bump_layer = material.get_layer_by_type(XmacMaterialLayerType::Specular);

        let gltf_diffuse_tex =
            xmac_mat_layer_to_texture(format!("{}_diffuse", material.name), diffuse_layer, output);

        let specular = if let Some(specular_layer) = specular_layer {
            let tex = xmac_mat_layer_to_texture(
                format!("{}_specular", material.name),
                specular_layer,
                output,
            );
            Some(Specular {
                specular_factor: SpecularFactor(material.shine_strength),
                specular_texture: Some(Info {
                    index: tex,
                    tex_coord: 0,
                    extensions: None,
                    extras: None,
                }),
                // specular_color_factor: SpecularColorFactor(
                //     Into::<Vector3>::into(&material.specular_color).into(),
                // ),
                specular_color_texture: Some(Info {
                    index: tex,
                    tex_coord: 0,
                    extensions: None,
                    extras: None,
                }),
                ..Default::default()
            })
        } else {
            None
        };

        let normal_texture = if let Some(bump_layer) = bump_layer {
            Some(NormalTexture {
                index: xmac_mat_layer_to_texture(
                    format!("{}_normal", material.name),
                    bump_layer,
                    output,
                ),
                scale: 1.0,
                tex_coord: 0,
                extensions: None,
                extras: None,
            })
        } else {
            None
        };

        let alpha_mode = match diffuse_layer.blend_mode {
            XmacLayerBlendMode::None => gltf::material::AlphaMode::Opaque,
            XmacLayerBlendMode::Over => gltf::material::AlphaMode::Blend,
            other => {
                println!("Warn: Blend Mode {other:?} is not supported in Gltf");
                gltf::material::AlphaMode::Blend
            }
        };

        let gltf_mat = GltfMaterial {
            alpha_mode: GltfValid(alpha_mode),
            double_sided: material.double_sided,
            name: Some(material.name.clone()),
            pbr_metallic_roughness: PbrMetallicRoughness {
                //base_color_factor: todo!(),
                base_color_texture: Some(Info {
                    index: gltf_diffuse_tex,
                    tex_coord: 0,
                    extensions: None,
                    extras: None,
                }),
                metallic_factor: StrengthFactor(0.0),
                //roughness_factor: todo!(),
                //metallic_roughness_texture: todo!(),
                ..PbrMetallicRoughness::default()
            },
            emissive_factor: EmissiveFactor(Into::<Vector3>::into(&material.emissive_color).into()),
            normal_texture,
            extensions: Some(Material {
                specular,
                ior: Some(Ior {
                    ior: IndexOfRefraction(material.refraction_index),
                    extras: None,
                }),
            }),
            extras: None,
            ..Default::default()
        };
        output.push(gltf_mat);
    }
    Ok(())
}

fn xmac_mat_layer_to_texture(
    name: String,
    layer: &XmacStandardMaterialLayer,
    output: &mut GltfRoot,
) -> GltfIndex<Texture> {
    let image = Image {
        buffer_view: None,
        mime_type: Some(MimeType("image/png".into())),
        name: Some(format!("{name}_img")),
        uri: Some(format!("{}.png", layer.texture)), //"testtex.png".into()), //
        extensions: None,
        extras: None,
    };
    let image = output.push(image);
    let tex = Texture {
        name: Some(name),
        sampler: None,
        source: image,
        extensions: None,
        extras: None,
    };
    // ToDo: actually export material texture to png there
    output.push(tex)
}
