use std::{collections::HashMap, time::SystemTime};

use formats::{
    types::{Mat4, Quat, Vec3, Vec4},
    xmac::{
        chunks::{
            mesh::XmacMesh,
            nodes::{XmacNode, XmacNodeFlags, XmacNodeId, XmacNodes},
            XmacChunk,
        },
        XmacFile,
    },
};

use gltf::{
    json::{accessor, validation::Checked as GltfChecked, Root as GltfRoot},
    scene::Transform,
};

use crate::ConvError;

use super::Result;

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
    file_time: SystemTime,
) -> Result<XmacFile> {
    let mut result = XmacFile::new(file_time);

    let mut tmp = TempData {
        node_mapping: HashMap::new(),
        mesh_nodes: HashMap::new(),
    };

    translate_nodes(&gltf, &mut tmp, &mut result)?;
    // translate_materials(input, &mut outputs)?;
    translate_meshes(&gltf, &buffer, &mut tmp, &mut result)?;
    // translate_skinning(input, &mut outputs)?;
    // translate_morphs(input, &mut outputs)?;

    //result.set_bounding

    Ok(result)
}

fn translate_nodes(gltf: &gltf::Document, tmp: &mut TempData, output: &mut XmacFile) -> Result<()> {
    let mut nodes = Vec::new();
    // maps child gltf id -> parent xmac id
    let mut parents = HashMap::new();
    let mut skeleton_nodes = gltf
        .skins()
        .flat_map(|s| s.skeleton())
        .map(|s| s.index())
        .collect::<Vec<_>>();
    skeleton_nodes.sort();
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
        if gltf_node.mesh().is_none() && skeleton_nodes.binary_search(&gltf_node_idx).is_err() {
            println!("Dropping Node {gltf_node_idx} ({name})");
        }

        let (mut local_scale, rotation, mut local_pos) = match gltf_node.transform() {
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

        local_scale = local_scale.recip();
        local_pos = local_pos * 100.0; //m to cm
        let parent_idx = parents.get(&gltf_node_idx).copied();

        let node = XmacNode {
            name,
            rotation,
            unknown1: Vec4::new(0.0, 0.0, 0.0, 1.0),
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

fn translate_meshes(
    gltf: &gltf::Document,
    buffer: &Vec<gltf::buffer::Data>,
    tmp: &mut TempData,
    output: &mut XmacFile,
) -> Result<()> {
    for mesh in gltf.meshes() {
        let mesh_idx = mesh.index();
        let mesh_node_idx = *tmp.mesh_nodes.get(&mesh_idx).unwrap();
        let mesh_node = &output.get_nodes_chunk().unwrap().nodes[mesh_node_idx];

        let mut vertex_attribute_layers = Vec::new();
        let mut submeshes = Vec::new();
        let mut orig_verts_count = 0;

        for prim in mesh.primitives() {
            if prim.mode() != gltf::json::mesh::Mode::Triangles {
                return Err(ConvError::NotImplemented(format!(
                    "Only Triangle meshes are supported, found {:?}",
                    prim.mode()
                )));
            }

            let mut read = prim.reader(|buf| buffer.get(buf.index()).map(|v| v.0.as_slice()));
            for (attr_type, attrib) in prim.attributes() {
                match attr_type {
                    gltf::Semantic::Positions => {
                        read.read_positions().unwrap();
                    }
                    _ => {}
                }
            }
        }

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
    }

    Ok(())
}
