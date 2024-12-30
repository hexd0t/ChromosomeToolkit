use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufWriter, Seek, Write},
    path::Path,
};

use super::{ConvError, Result};
use formats::{
    binimport::BinImport,
    helpers::{write_f32, write_u16, write_u32},
    types::{Mat4, Vec2, Vec3, Vec4},
    xmac::{
        chunks::{
            material::{XmacLayerBlendMode, XmacMaterialLayerType, XmacStandardMaterialLayer},
            mesh::{XmacMesh, XmacMeshSubmesh},
            morph_targets::MeshDeformDelta,
            skinning_info::XmacSkinningInfo,
        },
        XmacFile,
    },
};
use gltf::json::{
    accessor::{
        sparse::{Indices as GltfSparseIndices, Sparse as GltfSparse, Values as GltfSparseValues},
        IndexComponentType,
    },
    extensions::material::{IndexOfRefraction, Ior, Material, Specular, SpecularFactor},
    image::MimeType,
    material::{EmissiveFactor, NormalTexture, PbrMetallicRoughness, StrengthFactor},
    mesh::{MorphTarget, Primitive as GltfPrimitive},
    texture::Info,
    validation::Checked::Valid as GltfValid,
    Accessor as GltfAccessor, Buffer as GltfBuffer, Image, Index as GltfIndex,
    Material as GltfMaterial, Mesh as GltfMesh, Node as GltfNode, Root as GltfRoot,
    Skin as GltfSkin, Texture,
};
use serde_json::Map;

type MeshExtras = serde_json::Map<String, serde_json::Value>;

struct Outputs<'a> {
    gltf: &'a mut GltfRoot,
    buffer: GltfIndex<GltfBuffer>,
    buffer_file: BufWriter<File>,
    mesh_extras: Vec<MeshExtras>,
}

pub fn xmac_to_gltf(input: &XmacFile, buffer_path: &Path) -> Result<GltfRoot> {
    let mut gltf = GltfRoot::default();

    gltf.extensions_used.push("KHR_materials_ior".to_string());
    gltf.extensions_used
        .push("KHR_materials_specular".to_string());

    // Buffer for binary data:
    let (buffer, buffer_file) = create_buffer(&mut gltf, buffer_path)?;

    // It's easier to collect MeshExtras separately during translation:
    let mesh_extras: Vec<MeshExtras> = Vec::new();
    let mut outputs = Outputs {
        gltf: &mut gltf,
        buffer,
        buffer_file,
        mesh_extras,
    };

    translate_nodes(input, &mut outputs)?;
    translate_materials(input, &mut outputs)?;
    translate_meshes(input, &mut outputs)?;
    translate_skinning(input, &mut outputs)?;
    translate_morphs(input, &mut outputs)?;

    // Apply MeshExtras to GLTF data structure:
    apply_extras(&mut outputs)?;

    Ok(gltf)
}

fn create_buffer(
    output: &mut GltfRoot,
    buffer_path: &Path,
) -> Result<(GltfIndex<GltfBuffer>, BufWriter<File>)> {
    use gltf::json::validation::USize64;

    let buffer = output.push(GltfBuffer {
        byte_length: USize64(0),
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

    let buffer_file = File::create(buffer_path)?;
    let buffer_file = BufWriter::new(buffer_file);

    Ok((buffer, buffer_file))
}

fn apply_extras(outputs: &mut Outputs) -> Result<()> {
    for (mesh_idx, extras) in outputs
        .mesh_extras
        .drain(..)
        .enumerate()
        .filter(|(_, e)| !e.is_empty())
    {
        let raw_extras = serde_json::to_string(&serde_json::Value::Object(extras)).unwrap();
        outputs.gltf.meshes[mesh_idx].extras =
            Some(gltf::json::extras::RawValue::from_string(raw_extras).unwrap());
    }
    outputs.gltf.buffers[outputs.buffer.value()].byte_length =
        gltf::json::validation::USize64(outputs.buffer_file.stream_position()?);
    Ok(())
}

fn translate_nodes(input: &XmacFile, outputs: &mut Outputs) -> Result<()> {
    if let Some(nodes) = input.get_nodes_chunk() {
        for (node_idx, node) in nodes.into_iter().enumerate() {
            let mut pos = node.local_pos;
            let mut scale = node.local_scale;
            pos *= 0.01; // game is in cm, GLTF usually in m

            //for some reason, scale is inverted (maybe only when multiply-order == 1?):
            scale.x = 1.0 / scale.x;
            scale.y = 1.0 / scale.y;
            scale.z = 1.0 / scale.z;

            let transform =
                Mat4::from_rotation_translation(node.rotation, pos) * Mat4::from_scale(scale);

            let gltf_node = GltfNode {
                name: Some(node.name.clone()),
                matrix: Some(transform.to_cols_array()),

                ..GltfNode::default()
            };
            outputs.gltf.nodes.push(gltf_node);
            if let Some(parent_id) = node.parent_idx {
                if parent_id >= outputs.gltf.nodes.len() {
                    return Err(ConvError::NotImplemented(format!(
                        "Node {node_idx} encountered before its parent {parent_id}"
                    )));
                }
                let parent_children = outputs.gltf.nodes[parent_id]
                    .children
                    .get_or_insert_default();
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

fn translate_meshes(input: &XmacFile, outputs: &mut Outputs) -> Result<()> {
    for mesh in input.get_mesh_chunks() {
        let gltf_node = &mut outputs.gltf.nodes[mesh.node_id.0 as usize];
        let mesh_idx = outputs.gltf.meshes.len();
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
        let [positions, normals, tangents, uvs] = write_mesh_buffer(mesh, outputs)?;

        let mut primitives = Vec::with_capacity(mesh.submeshes.len());
        let mut index_offset = 0;
        for submesh in &mesh.submeshes {
            let indices = write_submesh_buffer(submesh, index_offset, outputs)?;
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

        outputs.gltf.meshes.push(gltf_mesh);
        outputs.mesh_extras.push(Map::new());
    }
    Ok(())
}

fn bounding_box(positions: &[Vec3]) -> ([f32; 3], [f32; 3]) {
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

fn write_mesh_buffer(
    mesh: &XmacMesh,
    outputs: &mut Outputs,
) -> Result<[GltfIndex<GltfAccessor>; 4]> {
    use gltf::json::validation::USize64;

    let mesh_positions = mesh.get_position_attrib().unwrap();
    let mesh_normals = mesh.get_normal_attrib().unwrap();
    let mesh_tangents = mesh.get_tangent_attrib().unwrap();
    let mesh_uvs = mesh.get_uv_attrib().unwrap();

    if mesh.vertex_attribute_layers.len() != 5 {
        //4 used here + orig_vert_idx in skinning
        println!("Warning: xmac has additional mesh attrib layers which will not be exported!");
    }

    const POSITION_SIZE: usize = std::mem::size_of::<Vec3>();
    const NORMAL_SIZE: usize = std::mem::size_of::<Vec3>();
    const TANGENT_SIZE: usize = std::mem::size_of::<Vec4>();
    const UV_SIZE: usize = std::mem::size_of::<Vec2>();
    const VERTEX_SIZE: usize = POSITION_SIZE + NORMAL_SIZE + TANGENT_SIZE + UV_SIZE;

    let vertex_count = mesh_positions.len();
    let start_offset = outputs.buffer_file.stream_position()?;

    let bounding = bounding_box(mesh_positions);

    let buffer_length = vertex_count * VERTEX_SIZE;
    let buffer_view = outputs.gltf.push(gltf::json::buffer::View {
        buffer: outputs.buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: Some(USize64(start_offset)),
        byte_stride: Some(gltf::json::buffer::Stride(VERTEX_SIZE)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(GltfValid(gltf::json::buffer::Target::ArrayBuffer)),
    });
    let gltf_positions = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(buffer_view),
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
    let gltf_normals = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(buffer_view),
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
    let gltf_tangents = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(buffer_view),
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
    let gltf_uv = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(buffer_view),
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

    for idx in 0..vertex_count {
        let mut pos = mesh_positions[idx];
        pos *= 0.01; //game is in cm, GLTF usually in m
        pos.save(&mut outputs.buffer_file)?;
        mesh_normals[idx].save(&mut outputs.buffer_file)?;
        mesh_tangents[idx].save(&mut outputs.buffer_file)?;
        mesh_uvs[idx].save(&mut outputs.buffer_file)?;
    }
    outputs.buffer_file.flush()?;

    Ok([gltf_positions, gltf_normals, gltf_tangents, gltf_uv])
}

fn write_submesh_buffer(
    submesh: &XmacMeshSubmesh,
    index_offset: u32,
    outputs: &mut Outputs,
) -> Result<GltfIndex<GltfAccessor>> {
    use gltf::json::validation::USize64;

    const INDEX_SIZE: usize = std::mem::size_of::<u32>();

    let start_offset = outputs.buffer_file.stream_position()?;
    let buffer_length = submesh.indices.len() * INDEX_SIZE;
    let buffer_view = outputs.gltf.push(gltf::json::buffer::View {
        buffer: outputs.buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: Some(USize64(start_offset)),
        byte_stride: None, //Non-Vertexbuffers must not have stride
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(GltfValid(gltf::json::buffer::Target::ElementArrayBuffer)),
    });
    let gltf_indices = outputs.gltf.push(GltfAccessor {
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

    for index in &submesh.indices {
        formats::helpers::write_u32(&mut outputs.buffer_file, *index + index_offset)?;
    }
    outputs.buffer_file.flush()?;

    Ok(gltf_indices)
}

fn translate_skinning(input: &XmacFile, outputs: &mut Outputs) -> Result<()> {
    for skin in input.get_skinning_chunks() {
        let node_id = skin.node_id;
        let skin_mesh = input.get_mesh_chunk(node_id).ok_or_else(|| {
            ConvError::MandatoryDataMissing(format!(
                "Skinning Info for {node_id:?} is missing Mesh data!"
            ))
        })?;
        let gltf_node = &outputs.gltf.nodes[node_id.0 as usize];
        let skin_name = gltf_node
            .name
            .as_ref()
            .map(|node_name| format!("{node_name}_skin",))
            .unwrap_or_else(|| format!("skin_{}", node_id.0));

        let (node_to_joint_idx, joints) = calculate_relevant_joint_ids(input, skin);

        let inverse_binds = calculate_inv_binds(outputs.gltf, &node_to_joint_idx, &joints);

        let [gltf_inversebinds, gltf_joints, gltf_weights] =
            write_skin_buffer(skin, skin_mesh, node_to_joint_idx, inverse_binds, outputs)?;

        let gltf_skin = GltfSkin {
            extensions: None,
            extras: None,
            inverse_bind_matrices: Some(gltf_inversebinds),
            joints,
            name: Some(skin_name),
            skeleton: None,
        };
        let gltf_skin = outputs.gltf.push(gltf_skin);
        let gltf_node = &mut outputs.gltf.nodes[node_id.0 as usize];
        gltf_node.skin = Some(gltf_skin);

        let gltf_mesh = get_gltf_mesh_for_node(outputs.gltf, node_id);
        for prim in gltf_mesh.primitives.iter_mut() {
            prim.attributes.insert(
                GltfValid(gltf::json::mesh::Semantic::Joints(0)),
                gltf_joints,
            );
            prim.attributes.insert(
                GltfValid(gltf::json::mesh::Semantic::Weights(0)),
                gltf_weights,
            );
        }
    }
    Ok(())
}

fn get_gltf_mesh_for_node(
    gltf: &mut GltfRoot,
    node_id: formats::xmac::chunks::nodes::XmacNodeId,
) -> &mut GltfMesh {
    let mesh_idx = get_gltf_mesh_idx_for_node(gltf, node_id);
    gltf.meshes.get_mut(mesh_idx).unwrap()
}

fn get_gltf_mesh_idx_for_node(
    gltf: &GltfRoot,
    node_id: formats::xmac::chunks::nodes::XmacNodeId,
) -> usize {
    gltf.nodes[node_id.0 as usize].mesh.unwrap().value()
}

fn calculate_inv_binds(
    gltf: &mut GltfRoot,
    node_to_joint_idx: &HashMap<u16, u16>,
    joints: &[GltfIndex<GltfNode>],
) -> Vec<Mat4> {
    let mut inverse_binds = vec![Mat4::IDENTITY; joints.len()];
    for (joint_idx, joint_node_idx) in joints.iter().enumerate() {
        let joint_node = &gltf.nodes[joint_node_idx.value()];
        inverse_binds[joint_idx] = Mat4::from_cols_array(joint_node.matrix.as_ref().unwrap())
            .inverse()
            * inverse_binds[joint_idx];
        for child_joint in joint_node
            .children
            .iter()
            .flat_map(|c| c.iter())
            .filter_map(|c_node_id| node_to_joint_idx.get(&(c_node_id.value() as u16)))
        {
            inverse_binds[*child_joint as usize] = inverse_binds[joint_idx];
        }
    }
    inverse_binds
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

fn write_skin_buffer(
    skin: &XmacSkinningInfo,
    skin_mesh: &XmacMesh,
    node_to_joint_idx: HashMap<u16, u16>,
    inverse_binds: Vec<Mat4>,
    outputs: &mut Outputs,
) -> Result<[GltfIndex<GltfAccessor>; 3]> {
    use gltf::json::validation::USize64;

    let vertex_to_orig_vertex = skin_mesh.get_orig_vert().unwrap();

    // glTf skinning is usually restricted to 4 influences:
    const JOINT_IDX_SIZE: usize = std::mem::size_of::<u16>() * 4;
    const JOINT_WEIGHT_SIZE: usize = std::mem::size_of::<f32>() * 4;
    const SKIN_ENTRY_SIZE: usize = JOINT_IDX_SIZE + JOINT_WEIGHT_SIZE;
    const INVERSE_BIND_SIZE: usize = std::mem::size_of::<Mat4>();

    let joint_count = node_to_joint_idx.len();
    let vertex_count = vertex_to_orig_vertex.len();

    let joints_length = joint_count * INVERSE_BIND_SIZE;
    let vertex_length = vertex_count * SKIN_ENTRY_SIZE;
    let start_offset = outputs.buffer_file.stream_position()?;
    let joints_buffer_view = outputs.gltf.push(gltf::json::buffer::View {
        buffer: outputs.buffer,
        byte_length: USize64::from(joints_length),
        byte_offset: Some(USize64(start_offset)),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: None,
    });
    let vertex_buffer_view = outputs.gltf.push(gltf::json::buffer::View {
        buffer: outputs.buffer,
        byte_length: USize64::from(vertex_length),
        byte_offset: Some(USize64(start_offset + joints_length as u64)),
        byte_stride: Some(gltf::json::buffer::Stride(SKIN_ENTRY_SIZE)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(GltfValid(gltf::json::buffer::Target::ArrayBuffer)),
    });

    let gltf_inverse_binds = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(joints_buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(joint_count),
        component_type: GltfValid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: GltfValid(gltf::json::accessor::Type::Mat4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let gltf_joints = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(vertex_buffer_view),
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
    let gltf_weights = outputs.gltf.push(GltfAccessor {
        buffer_view: Some(vertex_buffer_view),
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

    for inv_bind in inverse_binds {
        inv_bind.save(&mut outputs.buffer_file)?;
    }

    for orig_vertex in vertex_to_orig_vertex {
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
            write_u16(&mut outputs.buffer_file, joint)?;
        }
        for weight in weights.into_iter() {
            write_f32(&mut outputs.buffer_file, weight)?;
        }
    }

    outputs.buffer_file.flush()?;

    Ok([gltf_inverse_binds, gltf_joints, gltf_weights])
}

fn translate_materials(input: &XmacFile, outputs: &mut Outputs) -> Result<()> {
    for material in input.get_material_chunks() {
        let diffuse_layer = material
            .get_layer_by_type(XmacMaterialLayerType::Diffuse)
            .unwrap();
        let specular_layer = material.get_layer_by_type(XmacMaterialLayerType::Bump);
        let bump_layer = material.get_layer_by_type(XmacMaterialLayerType::Specular);

        let gltf_diffuse_tex = xmac_mat_layer_to_texture(
            format!("{}_diffuse", material.name),
            diffuse_layer,
            outputs.gltf,
        );

        let specular = if let Some(specular_layer) = specular_layer {
            let tex = xmac_mat_layer_to_texture(
                format!("{}_specular", material.name),
                specular_layer,
                outputs.gltf,
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
                //     Into::<Vec3>::into(&material.specular_color).into(),
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
                    outputs.gltf,
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
            emissive_factor: EmissiveFactor(material.emissive_color.truncate().into()),
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
        outputs.gltf.push(gltf_mat);
    }
    Ok(())
}

fn xmac_mat_layer_to_texture(
    name: String,
    layer: &XmacStandardMaterialLayer,
    gltf: &mut GltfRoot,
) -> GltfIndex<Texture> {
    let image = Image {
        buffer_view: None,
        mime_type: Some(MimeType("image/png".into())),
        name: Some(format!("{name}_img")),
        uri: Some(format!("{}.png", layer.texture)), //"testtex.png".into()), //
        extensions: None,
        extras: None,
    };
    let image = gltf.push(image);
    let tex = Texture {
        name: Some(name),
        sampler: None,
        source: image,
        extensions: None,
        extras: None,
    };
    // ToDo: actually export material texture to png there
    gltf.push(tex)
}

fn translate_morphs(input: &XmacFile, outputs: &mut Outputs) -> Result<()> {
    for morph in input
        .get_morph_chunk()
        .iter()
        .flat_map(|c| c.targets.iter())
    {
        for deform in &morph.mesh_deform_deltas {
            let target_node_idx = deform.node_id;

            let target_mesh = input.get_mesh_chunk(target_node_idx).unwrap();
            let [morph_pos, morph_norm, morph_tang] = write_morph_buffer(
                target_mesh.get_position_attrib().unwrap().len(),
                &deform.deltas,
                outputs,
            )?;
            let gltf_target = MorphTarget {
                positions: Some(morph_pos),
                normals: Some(morph_norm),
                tangents: Some(morph_tang),
            };

            for (submesh_idx, _submesh) in target_mesh.submeshes.iter().enumerate() {
                let gltf_mesh = get_gltf_mesh_for_node(outputs.gltf, target_node_idx);
                let prim_targets = gltf_mesh.primitives[submesh_idx]
                    .targets
                    .get_or_insert_default();

                prim_targets.push(gltf_target.clone());
            }

            let target_mesh_idx = get_gltf_mesh_idx_for_node(outputs.gltf, target_node_idx);
            let mesh_target_names = outputs.mesh_extras[target_mesh_idx]
                .entry("targetNames".to_string())
                .or_insert_with(|| serde_json::Value::Array(Vec::new()))
                .as_array_mut()
                .unwrap();
            mesh_target_names.push(morph.name.clone().into());
            let gltf_mesh = get_gltf_mesh_for_node(outputs.gltf, target_node_idx);
            gltf_mesh.weights.get_or_insert_default().push(0.0);
        }
    }
    Ok(())
}

fn write_morph_buffer(
    mesh_vert_count: usize,
    deforms: &[MeshDeformDelta],
    outputs: &mut Outputs,
) -> Result<[GltfIndex<GltfAccessor>; 3]> {
    use gltf::json::validation::USize64;

    const VERTEX_ID_SIZE: usize = std::mem::size_of::<u32>();

    const POSITION_SIZE: usize = std::mem::size_of::<Vec3>();
    const NORMAL_SIZE: usize = std::mem::size_of::<Vec3>();
    const TANGENT_SIZE: usize = std::mem::size_of::<Vec3>();
    const VERTEX_SIZE: usize = POSITION_SIZE + NORMAL_SIZE + TANGENT_SIZE;

    let vertex_count = deforms.len();
    let bounding = bounding_box(
        &deforms
            .iter()
            .map(|d| d.position_delta)
            .chain(std::iter::once(Vec3 {
                //The bounding is defined on the whole vector, including the sparse zero deltas
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }))
            .collect::<Vec<_>>(),
    );

    let idx_buffer_length = vertex_count * VERTEX_ID_SIZE;
    let vert_buffer_length = vertex_count * VERTEX_SIZE;
    let start_offset = outputs.buffer_file.stream_position()?;

    let idx_buffer_view = outputs.gltf.push(gltf::json::buffer::View {
        buffer: outputs.buffer,
        byte_length: USize64(idx_buffer_length as u64),
        byte_offset: Some(USize64(start_offset)),
        byte_stride: None,
        extensions: None,
        extras: None,
        name: None,
        target: None,
    });
    // Sparse value views may not define stride or target
    let vert_buffer_view = outputs.gltf.push(gltf::json::buffer::View {
        buffer: outputs.buffer,
        byte_length: USize64::from(vert_buffer_length),
        byte_offset: Some(USize64(start_offset + idx_buffer_length as u64)),
        byte_stride: None,
        extensions: None,
        extras: None,
        name: None,
        target: None,
    });

    let sparse_indices = GltfSparseIndices {
        buffer_view: idx_buffer_view,
        byte_offset: USize64(0),
        component_type: GltfValid(IndexComponentType(gltf::json::accessor::ComponentType::U32)),
        extensions: None,
        extras: None,
    };
    let gltf_positions = outputs.gltf.push(GltfAccessor {
        buffer_view: None,
        byte_offset: None,
        count: USize64::from(mesh_vert_count),
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
        sparse: Some(GltfSparse {
            count: USize64::from(vertex_count),
            indices: sparse_indices.clone(),
            values: GltfSparseValues {
                buffer_view: vert_buffer_view,
                byte_offset: USize64(0),
                extensions: None,
                extras: None,
            },
            extensions: None,
            extras: None,
        }),
    });
    let gltf_normals = outputs.gltf.push(GltfAccessor {
        buffer_view: None,
        byte_offset: None,
        count: USize64::from(mesh_vert_count),
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
        sparse: Some(GltfSparse {
            count: USize64::from(vertex_count),
            indices: sparse_indices.clone(),
            values: GltfSparseValues {
                buffer_view: vert_buffer_view,
                byte_offset: USize64::from(POSITION_SIZE * vertex_count),
                extensions: None,
                extras: None,
            },
            extensions: None,
            extras: None,
        }),
    });
    let gltf_tangents = outputs.gltf.push(GltfAccessor {
        buffer_view: None,
        byte_offset: None,
        count: USize64::from(mesh_vert_count),
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
        sparse: Some(GltfSparse {
            count: USize64::from(vertex_count),
            indices: sparse_indices.clone(),
            values: GltfSparseValues {
                buffer_view: vert_buffer_view,
                byte_offset: USize64::from(
                    POSITION_SIZE * vertex_count + TANGENT_SIZE * vertex_count,
                ),
                extensions: None,
                extras: None,
            },
            extensions: None,
            extras: None,
        }),
    });

    for vert in deforms {
        write_u32(&mut outputs.buffer_file, vert.vertex_id)?;
    }
    for vert in deforms {
        let mut pos = vert.position_delta;
        pos *= 0.01; //game is in cm, GLTF usually in m
        pos.save(&mut outputs.buffer_file)?;
    }
    for vert in deforms {
        vert.normal_delta.save(&mut outputs.buffer_file)?;
    }
    for vert in deforms {
        vert.tangent_delta.save(&mut outputs.buffer_file)?;
    }
    outputs.buffer_file.flush()?;

    Ok([gltf_positions, gltf_normals, gltf_tangents])
}
