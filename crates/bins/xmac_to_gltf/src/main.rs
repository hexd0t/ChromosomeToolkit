use std::{
    collections::{BTreeMap, VecDeque},
    env,
    ffi::OsString,
    io::{BufWriter, Seek, Write},
    path::Path,
};

use formats::{
    types::{
        properties::{PropData, Property},
        BoundingBox, Vector2, Vector3, Vector4,
    },
    xmac::{
        chunks::{
            mesh::{XmacMesh, XmacMeshAttrib, XmacMeshAttribLayer, XmacMeshSubmesh},
            nodes::{XmacNode, XmacNodeId, XmacNodes},
            XmacChunk,
        },
        XmacFile,
    },
};
use gltf::json::{
    mesh::Primitive as GltfPrimitive, validation::Checked::Valid as GltfValid,
    Accessor as GltfAccessor, Buffer as GltfBuffer, Index as GltfIndex, Mesh as GltfMesh,
    Node as GltfNode, Root as GltfRoot,
};
//use serde::Serialize;

fn main() {
    println!("Chromosome Toolkit - R1 - XMAC to USD");
    let mut queue = env::args().skip(1).collect::<VecDeque<_>>();
    while let Some(arg) = queue.pop_front() {
        println!("{}", arg);
        let os_arg = OsString::from(&arg);
        let path = Path::new(&os_arg);
        if !path.exists() {
            println!("not found");
            continue;
        }

        if path.is_dir() {
            if let Ok(dir) = path.read_dir() {
                for file in dir.flatten() {
                    let meta = file.metadata().unwrap();
                    let path = file.path().to_string_lossy().to_string();
                    if meta.is_dir() || path.ends_with("._xmac") {
                        queue.push_back(path);
                    }
                }
            } else {
                println!("Reading dir failed");
            }
            continue;
        }
        let in_data = std::fs::File::open(path).unwrap();
        let mut in_data = std::io::BufReader::new(in_data);
        let xmac = XmacFile::load(&mut in_data).unwrap();
        println!(
            "Read: {:x}/{:x}",
            in_data.stream_position().unwrap(),
            in_data.seek(std::io::SeekFrom::End(0)).unwrap()
        );
        drop(in_data);

        let out_arg = arg.replace("._xmac", "._xmac.gltf");
        if out_arg == arg {
            panic!("In == out path");
        }

        let gltf = xmac_to_gltf(&xmac, &arg).unwrap();
        println!("Translation done");

        let out_os = OsString::from(&out_arg);
        let out_path = std::path::Path::new(&out_os);

        let out_file = std::fs::File::create(out_path).expect("Unable to open output file");
        let mut out_file = std::io::BufWriter::new(out_file);

        // let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        // let mut ser = serde_json::Serializer::with_formatter(&mut out_file, formatter);
        // xmac.serialize(&mut ser).unwrap();
        gltf.to_writer_pretty(&mut out_file).unwrap();
        out_file.flush().unwrap();
        println!("done");
    }
}

fn xmac_to_gltf(input: &XmacFile, src_filepath: &str) -> Result<GltfRoot> {
    let mut output = GltfRoot::default();

    let bounding_box = get_bounding_box(input)?;
    translate_nodes(input, &mut output)?;
    translate_meshes(input, &mut output, src_filepath)?;

    Ok(output)
}

fn translate_nodes(input: &XmacFile, output: &mut GltfRoot) -> Result<()> {
    fn get_nodes_chunk(chunk: &XmacChunk) -> Option<&XmacNodes> {
        if let XmacChunk::Nodes(nodes) = chunk {
            Some(nodes)
        } else {
            None
        }
    }
    if let Some(nodes) = input.chunks.iter().find_map(get_nodes_chunk) {
        for (node_idx, node) in nodes.into_iter().enumerate() {
            let scale = if node.parent_idx.is_none() {
                let mut scale = node.local_scale.clone();
                scale.y *= -1.0; //Flip Y-Axis for root nodes to reflect LHS vs RHS
                scale
            } else {
                node.local_scale.clone()
            };
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

fn get_bounding_box(input: &XmacFile) -> Result<&BoundingBox> {
    fn get_boundary_prop(prop: &Property) -> Option<&BoundingBox> {
        if &prop.name != "Boundary" {
            None
        } else {
            if let PropData::BoundingBox(bb) = prop.data.as_ref() {
                Some(bb)
            } else {
                println!("Warn: Boundary prop doesn't contain BoundingBox data!");
                None
            }
        }
    }
    if let Some(boundary) = input.props.iter().find_map(get_boundary_prop) {
        Ok(boundary)
    } else {
        Err(ConvError::NotImplemented(
            "Boundary prop missing; calculation is not implemented".to_string(),
        ))
    }
}

fn translate_meshes(input: &XmacFile, output: &mut GltfRoot, src_filepath: &str) -> Result<()> {
    fn get_mesh_chunk(chunk: &XmacChunk) -> Option<&XmacMesh> {
        if let XmacChunk::Mesh(mesh) = chunk {
            Some(mesh)
        } else {
            None
        }
    }
    for mesh in input.chunks.iter().filter_map(get_mesh_chunk) {
        let gltf_node = &mut output.nodes[mesh.node_id.0 as usize];
        let mesh_idx = output.meshes.len();
        gltf_node.mesh = Some(GltfIndex::new(mesh_idx as u32));

        let mesh_name = gltf_node
            .name
            .as_ref()
            .map(|node_name| format!("{node_name}_mesh",))
            .unwrap_or_else(|| format!("mesh_{mesh_idx}"));
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
                material: None, // ToDo
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

    const POSITION_SIZE: usize = std::mem::size_of::<Vector3>();
    const NORMAL_SIZE: usize = std::mem::size_of::<Vector3>();
    const TANGENT_SIZE: usize = std::mem::size_of::<Vector4>();
    const UV_SIZE: usize = std::mem::size_of::<Vector2>();
    const VERTEX_SIZE: usize = POSITION_SIZE + NORMAL_SIZE + TANGENT_SIZE + UV_SIZE;

    let vertex_count = mesh_positions.len();

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
        min: None, // TODO Some(gltf::json::Value::Array(Vec::from(min))),
        max: None, // TODO Some(gltf::json::Value::Array(Vec::from(max))),
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
        byte_stride: Some(gltf::json::buffer::Stride(INDEX_SIZE)),
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

#[derive(Debug)]
pub enum ConvError {
    NotImplemented(String),
    MandatoryDataMissing(String),
    IoError(std::io::Error),
}

type Result<T> = std::result::Result<T, ConvError>;

impl std::error::Error for ConvError {}
impl std::fmt::Display for ConvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<std::io::Error> for ConvError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
