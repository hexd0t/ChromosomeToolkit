//! Resources on the XMAC format:
//!  - O3DE (formerly Amazon Lumberyard) has an importer for XMAC, although a newer version
//!    at https://github.com/o3de/o3de/tree/development/Gems/EMotionFX/Code/EMotionFX/Source
//!    older commits contain more 'legacy' RemoteFX code
//!    (since Amazon bought EmotionFX & employs their devs, this can be considered 'ground truth')
//!    License: Apache 2 or MIT
//!  - Lumberyard's archived repo has more old pieces of RemoteFX left:
//!    at github.com/aws/lumberyard/blob/master/dev/Gems/EMotionFX/Code/EMotionFX/Source/Importer/ActorFileFormat.h
//!    License: AWS Agreement
//!  - RisenEditor:
//!    at https://github.com/hhergeth/RisenEditor
//!    License: none
//!  - Baltram's rmTools:
//!    at https://github.com/Baltram/rmtools/blob/master/rmStuff/rmXmacReader.cpp
//!    License: GPLv3

use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use crate::{
    archive::ArchiveReadTarget,
    error::*,
    helpers::*,
    types::{properties::Property, DateTime, Matrix, Quaternion, Vector2, Vector3, Vector4},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacFile {
    pub timestamp: DateTime,
    pub props: Vec<Property>,
    pub chunks: Vec<XmacChunk>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum XmacChunk {
    Info(XmacInfo),
    Nodes(XmacNodes),
    MaterialInfo(XmacMaterialInfo),
    StdMaterial(XmacStdMaterial),
    Mesh(XmacMesh),
    Unknown(XmacUnknownChunk),
}

struct XmacChunkMeta {
    type_id: u32,
    size: u32,
    version: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacUnknownChunk {
    pub type_id: u32,
    pub version: u32,
    #[serde(with = "crate::helpers::ser_hex")]
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacInfo {
    /// the number of level of details - R1 doesn't seem to have LOD fields though
    // pub lod_count: u32,
    // TODO: The version used by R1 carries only one of these:
    /// the node number of the trajectory node used for motion extraction
    pub motion_extraction_node_index: i32,
    /// the retargeting root node index, most likely pointing to the hip or pelvis or invalid index (.1) when not set
    pub retarget_root_node_index: i32,
    /// supposedly contains unit_type (feet, cm, m, ...) and exporter version, but none of those are used
    pub unknown1: u32,
    pub unknown2: u32,
    /// source application (e.g. "3ds Max 2011", "Maya 2011")
    pub source_application: String,
    /// original filename of the 3dsMax/Maya file
    pub orig_filename: String,
    /// compilation date of the exporter
    pub exporter_date: String,
    /// the name of the actor
    pub actor_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacNodes {
    nodes: Vec<XmacNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacNode {
    name: String,
    rotation: Quaternion,
    unknown1: Vector4,
    local_pos: Vector3,
    local_scale: Vector3,
    unknown2: Vector3,
    unknown3: Vector2,
    parent_idx: Option<usize>,
    child_count: u32,
    flags: XmacNodeFlags,
    unknown4: f32,
    oriented_bounding_box: Matrix,
}

bitflags! {
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct XmacNodeFlags: u8 {
        /// Specifies whether we have to include this node in the bounds calculation or not (true on default).
        const IncludeInBoundsCalc = 0x1;
        /// Indicates if this node is an attachment node or not (false on default).
        const Attachment = 0x2;
        /// Indicates if this node is a critical node. A critical node is always included the skeleton and cannot be optimized out (false on default).
        const Critical = 0x4;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMaterialInfo {
    std_materials: usize,
    /// might also be generic materials instead
    fx_materials: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacStdMaterial {
    name: String,
    layers: Vec<XmacStandardMaterialLayer>,

    ambient_color: Vector4,
    diffuse_color: Vector4,
    specular_color: Vector4,
    emissive_color: Vector4,
    shine: f32,
    shine_strength: f32,
    opacity: f32,
    refraction_index: f32,
    double_sided: bool,
    wireframe: bool,
    transparency_type: XmacMaterialTransparencyType,
}

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacMaterialTransparencyType {
    Filter = b'F',
    Subtractive = b'S',
    Additive = b'A',
    Unknown = b'U',
}

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacLayerBlendMode {
    /// The foreground texture covers up the background texture entirely.
    None = 0,
    /// The foreground texture is applied like a decal to the background.
    /// The shape of the decal is determined by the foreground alpha.
    Over = 1,
    /// The result is the background texture cut in the shape of the foreground alpha.
    In = 2,
    /// The result is the opposite of In.
    /// It is as if the shape of the foreground alpha has been cut out of the background.
    Out = 3,
    /// The result color is the foreground color added to the background color as if being projected on the background through a slide projector.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Add = 4,
    /// The result color is the foreground color subtracted from the background color.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Subtract = 5,
    /// The result color is the foreground color multiplied by the background color.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Multiply = 6,
    /// The result color is the difference between the foreground color and the background color.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Difference = 7,
    /// The result color of each pixel is the background color or foreground color, whichever is lighter.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Lighten = 8,
    /// The result color of each pixel is the background color or foreground color, whichever is darker.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Darken = 9,
    /// The result color is the background color with saturation increased in proportion to the foreground color scaled by foreground alpha.
    /// If the foreground color is red, for example, the result color will be the background color with more saturated reds.
    Saturate = 10,
    /// The result color is the background color with saturation decreased in proportion to the foreground color scaled by foreground alpha.
    /// If the foreground color is red, for example, the result color will be the background color with desaturated reds.
    Desaturate = 11,
    /// The result color is the background color mixed with the foreground color, brighter where the foreground is bright and darker where the foreground is dark.
    /// It is as if the foreground texture represents the light falling on the background.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Illuminate = 12,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacStandardMaterialLayer {
    #[serde(rename = "type")]
    ty: XmacMaterialLayerType,
    texture: String,

    amount: f32,
    u_offset: f32,
    v_offset: f32,
    u_tiling: f32,
    v_tiling: f32,
    rotation_rads: f32,
    material_id: u16,
    blend_mode: XmacLayerBlendMode,
}

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacMaterialLayerType {
    Unknown = 0,
    Ambient = 1,
    Diffuse = 2,
    Specular = 3,
    Opacity = 4,
    /// Contains a normal-map
    Bump = 5,
    SelfIllumination = 6,
    /// shininess (for specular)
    Shine = 7,
    /// shine strength (for specular)
    ShineStrength = 8,
    FilterColor = 9,
    Reflect = 10,
    Refract = 11,
    Environment = 12,
    Displacement = 13,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMesh {
    vertex_attribute_layers: Vec<XmacMeshAttribLayer>,
    submeshes: Vec<XmacMeshSubmesh>,

    node_id: u32,
    unknown: u32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMeshAttribLayer {
    attribs: XmacMeshAttrib,
    unknown1: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum XmacMeshAttrib {
    Positions(Vec<Vector3>),
    Normals(Vec<Vector3>),
    Tangents(Vec<Vector4>),
    UvCoords(Vec<Vector2>),
    /// Contains a 4-byte RGBA value
    Colors32(Vec<u32>),
    OriginalVertexNumbers(Vec<u32>),
    /// Contains 4 f32 color entries (RGBA)
    Colors128(Vec<Vector4>),
    BiTangents(Vec<Vector3>),
    ClothData(Vec<u32>),
}

#[repr(u32)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacMeshAttribLayerType {
    /// Contains a Vector3
    Positions = 0,
    /// Contains a Vector3
    Normals = 1,
    /// Contains a Vector4
    Tangents = 2,
    /// Contains a Vector2
    UvCoords = 3,
    /// Contains a 4-byte RGBA value
    Colors32 = 4,
    /// Contains a u32
    OriginalVertexNumbers = 5,
    /// Contains 4 f32 color entries (RGBA)
    Colors128 = 6,
    /// Contains a Vector3
    BiTangents = 7,
    /// Contains a u32
    ClothData = 8,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMeshSubmesh {
    indices: Vec<u32>,
    bones: Vec<u32>,

    vertices_count: u32,
    material_idx: u32,
}

#[repr(u32)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]

pub enum XmacChunkType {
    Node = 0,
    Mesh = 1,
    SkinningInfo = 2,
    /// R1's data has invalid chunk sizes for these for some reason...
    StdMaterial = 3,
    StdMaterialLayer = 4,
    FxMaterial = 5,
    Limit = 6,
    Info = 7,
    MeshLodLevels = 8,
    StdProgMorphTarget = 9,
    NodeGroups = 10,
    Nodes = 11,
    StdPMorphTargets = 12,
    MaterialInfo = 13,
    NodeMotionSources = 14,
    AttachmentNodes = 15,
    MaterialAttributeSet = 16,
    GenericMaterial = 17,
    PhysicsSetup = 18,
    SimulatedObjectSetup = 19,
    MeshAsset = 20,
}

impl XmacFile {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let mut revision = [0u8; 4];
        src.read_exact(&mut revision)?;
        assert_eq!(&revision, "GR01".as_bytes());
        let mut data_revision = [0u8; 4];
        src.read_exact(&mut data_revision)?;
        assert_eq!(&data_revision, "MA02".as_bytes());

        //prop_offset: u32 - Header up to that point is always 0x28
        let prop_offset = read_u32(src)? as usize;
        assert_eq!(prop_offset, 0x28);

        let prop_length = read_u32(src)? as usize;

        let data_offset = read_u32(src)? as usize;
        assert_eq!(prop_length + prop_offset, data_offset);

        let _data_len = read_u32(src)? as usize;

        let timestamp = DateTime::load(src)?;

        let mut raw_file_ext = [0u8; 8];
        src.read_exact(&mut raw_file_ext)?;
        assert_eq!(&raw_file_ext[0..4], ".xac".as_bytes());
        assert_eq!(&raw_file_ext[4..8], &[0, 0, 0, 0]);

        let props = Self::load_meta(src)?;
        let chunks = Self::load_xmac(src)?;

        Ok(XmacFile {
            props,
            chunks,
            timestamp,
        })
    }

    fn load_meta<R: ArchiveReadTarget>(src: &mut R) -> Result<Vec<Property>> {
        let mut header = [0u8; 6];
        src.read_exact(&mut header)?;
        assert_eq!(&header, &[1, 0, 1, 1, 0, 1]);

        let class_name = src.read_str()?;
        assert_eq!(class_name.as_str(), "eCMotionActorResource2");
        let mut unknown1 = [0u8; 3];
        src.read_exact(&mut unknown1)?;
        assert_eq!(&unknown1, &[1, 0, 0]);

        let class_ver = read_u16(src)?;
        assert_eq!(class_ver, 201);
        let version = read_u16(src)?;
        assert_eq!(version, 201);
        let _data_len = read_u32(src)?;

        let prop_data_ver = read_u16(src)?;
        assert_eq!(prop_data_ver, 201);
        let prop_count = read_u32(src)? as usize;
        let mut props = Vec::with_capacity(prop_count);
        for _idx in 0..prop_count {
            props.push(Property::load(src)?);
        }
        let class_version = read_u16(src)?;
        assert_eq!(class_version, 201);
        Ok(props)
    }

    fn load_xmac<R: ArchiveReadTarget>(src: &mut R) -> Result<Vec<XmacChunk>> {
        let data_len = read_u32(src)? as u64;
        let xmac_start = src.stream_position()?;
        let xmac_finish = xmac_start + data_len;

        let mut magic = vec![0; 4];
        src.read_exact(&mut magic)?;
        assert_eq!(&magic, "XAC ".as_bytes());

        let _actor_version_maj = read_u8(src)?;
        let _actor_version_min = read_u8(src)?;
        // The version check in R1 is broken -
        // it is supposed to only accept 3.0, but accepts anything
        // with either maj = 3 OR min = 0 -
        // and the shipped files have V1.0 oO

        let big_endian = read_bool(src)?;
        let multiply_order = read_bool(src)?;

        let mut chunks = Vec::new();
        while src.stream_position()? < xmac_finish {
            let new_chunk = XmacChunk::load(src, big_endian, multiply_order, &chunks)?;
            chunks.push(new_chunk);
        }

        Ok(chunks)
    }
}

impl XmacChunk {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        multiply_order: bool,
        _prev_chunks: &[XmacChunk],
    ) -> Result<Self> {
        let chunk_meta = XmacChunkMeta::load(src, big_endian)?;
        let chunk_start = src.stream_position()?;
        let chunk_end = chunk_start + chunk_meta.size as u64;
        let result = if let Ok(chunk_type) = chunk_meta.type_id.try_into() {
            match chunk_type {
                XmacChunkType::Info => Self::handle_unknown(
                    XmacInfo::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::Info),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::Nodes => Self::handle_unknown(
                    XmacNodes::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::Nodes),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::MaterialInfo => Self::handle_unknown(
                    XmacMaterialInfo::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::MaterialInfo),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::StdMaterial => Self::handle_unknown(
                    XmacStdMaterial::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::StdMaterial),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::Mesh => Self::handle_unknown(
                    XmacMesh::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::Mesh),
                    src,
                    &chunk_meta,
                ),
                _ => {
                    println!(
                        "Unimplemented XMAC chunk {chunk_type:?}.{}@{:x}",
                        chunk_meta.version,
                        src.stream_position()?
                    );
                    XmacChunk::Unknown(XmacUnknownChunk::load(src, &chunk_meta)?)
                }
            }
        } else {
            println!(
                "Unknown XMAC chunk id {}.{}@{:x}, skipping",
                chunk_meta.type_id,
                chunk_meta.version,
                src.stream_position()?
            );
            XmacChunk::Unknown(XmacUnknownChunk::load(src, &chunk_meta)?)
        };
        let end_pos = src.stream_position()?;
        if end_pos != chunk_end {
            println!(
                "Chunk did not read exactly it's announced size, finished at 0x{end_pos:x} vs expected 0x{chunk_start:x} to 0x{chunk_end:x} (diff: {})!",
                end_pos - chunk_end
            );
            if chunk_meta.type_id == 3 {
                println!("(This is known for R1 StdMaterials)");
            }
        }
        Ok(result)
    }

    fn handle_unknown<R: ArchiveReadTarget>(
        parse_result: Option<XmacChunk>,
        src: &mut R,
        chunk_meta: &XmacChunkMeta,
    ) -> XmacChunk {
        parse_result
            .unwrap_or_else(|| XmacChunk::Unknown(XmacUnknownChunk::load(src, chunk_meta).unwrap()))
    }

    // fn get_last_material_info(prev_chunks: &[XmacChunk]) -> Result<&XmacMaterialInfo> {
    //     prev_chunks
    //         .iter()
    //         .rev()
    //         .find_map(|c| {
    //             if let XmacChunk::MaterialInfo(mat) = c {
    //                 Some(mat)
    //             } else {
    //                 None
    //             }
    //         })
    //         .ok_or_else(|| {
    //             Error::InvalidStructure(
    //                 "StdMaterials chunk without preceding MaterialInfo".to_string(),
    //             )
    //         })
    // }
}

impl XmacChunkMeta {
    fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let type_id = read_u32_endian(src, big_endian)?;
        let size = read_u32_endian(src, big_endian)?;
        let version = read_u32_endian(src, big_endian)?;
        Ok(Self {
            type_id,
            size,
            version,
        })
    }
}

impl XmacUnknownChunk {
    fn load<R: ArchiveReadTarget>(src: &mut R, chunk_meta: &XmacChunkMeta) -> Result<Self> {
        let mut data = vec![0; chunk_meta.size as usize];
        src.read_exact(&mut data)?;
        Ok(Self {
            type_id: chunk_meta.type_id,
            version: chunk_meta.version,
            data,
        })
    }
}

impl XmacInfo {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading INFO chunk...");
        match chunk_meta.version {
            2 => Ok(Some(XmacInfo {
                //lod_count: read_u32_endian(src, big_endian)?,
                motion_extraction_node_index: read_i32_endian(src, big_endian)?,
                retarget_root_node_index: read_i32_endian(src, big_endian)?,
                unknown1: read_u32_endian(src, big_endian)?,
                unknown2: read_u32_endian(src, big_endian)?,
                source_application: read_xmac_str(src, big_endian)?,
                orig_filename: read_xmac_str(src, big_endian)?,
                exporter_date: read_xmac_str(src, big_endian)?,
                actor_name: read_xmac_str(src, big_endian)?,
            })),
            ver => {
                println!(
                    "Unknown XMAC info version {ver}@{:x}",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}

impl XmacNodes {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading NODES chunk...");
        match chunk_meta.version {
            1 => {
                let node_count = read_u32_endian(src, big_endian)? as usize;
                let root_count = read_u32_endian(src, big_endian)? as usize;
                let mut nodes = Vec::with_capacity(node_count);
                for _idx in 0..node_count {
                    let rotation = Quaternion::load_endian(src, big_endian)?;
                    let unknown1 = Vector4::load_endian(src, big_endian)?;
                    let local_pos = Vector3::load_endian(src, big_endian)?;
                    let local_scale = Vector3::load_endian(src, big_endian)?;
                    let unknown2 = Vector3::load_endian(src, big_endian)?;
                    let unknown3 = Vector2::load_endian(src, big_endian)?;

                    let parent_idx = read_i32_endian(src, big_endian)?;
                    let parent_idx = if parent_idx == -1 {
                        None
                    } else {
                        Some(parent_idx as usize)
                    };
                    let child_count = read_u32_endian(src, big_endian)?;

                    let flags = read_u8(src)?;
                    let flags = XmacNodeFlags::from_bits(flags).ok_or(Error::EnumUnparsable(
                        format!("Parsing XmacNodeFlags failed, invalid value {flags:02x}"),
                    ))?;

                    for _idx in 0..3 {
                        let pad = read_u8(src)?;
                        assert_eq!(pad, 0);
                    }

                    // might be the other way around:
                    let unknown4 = read_f32_endian(src, big_endian)?;
                    let oriented_bounding_box = Matrix::load(src)?;

                    let node_name = read_xmac_str(src, big_endian)?;

                    nodes.push(XmacNode {
                        name: node_name,
                        rotation,
                        unknown1,
                        local_pos,
                        local_scale,
                        unknown2,
                        unknown3,
                        parent_idx,
                        child_count,
                        flags,
                        unknown4,
                        oriented_bounding_box,
                    });
                }
                let actual_root_nodes = nodes.iter().filter(|n| n.parent_idx.is_none()).count();
                if root_count != actual_root_nodes {
                    println!(
                        "Warning: Expected {root_count} root nodes, but found {actual_root_nodes}!"
                    );
                }
                Ok(Some(XmacNodes { nodes }))
            }
            version => {
                println!(
                    "Unknown XMAC nodes version {version}@{:x}",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}

impl XmacMaterialInfo {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading MATERIAL INFO chunk...");
        match chunk_meta.version {
            1 => {
                let total_materials = read_u32_endian(src, big_endian)? as usize;
                let std_materials = read_u32_endian(src, big_endian)? as usize;
                let fx_materials = read_u32_endian(src, big_endian)? as usize;

                if total_materials != std_materials {
                    return Err(Error::InvalidStructure(
                        "Non Std-Materials are not supported right now".to_string(),
                    ));
                }

                Ok(Some(Self {
                    std_materials,
                    fx_materials,
                }))
            }
            ver => {
                println!(
                    "Unknown XMAC material info version {ver}@{:x}, skipping",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}

impl XmacStdMaterial {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading STD MATERIAL chunk...");
        match chunk_meta.version {
            2 => {
                let ambient_color = Vector4::load_endian(src, big_endian)?;
                let diffuse_color = Vector4::load_endian(src, big_endian)?;
                let specular_color = Vector4::load_endian(src, big_endian)?;
                let emissive_color = Vector4::load_endian(src, big_endian)?;
                let shine = read_f32_endian(src, big_endian)?;
                let shine_strength = read_f32_endian(src, big_endian)?;
                let opacity = read_f32_endian(src, big_endian)?;
                let refraction_index = read_f32_endian(src, big_endian)?;
                let double_sided = read_bool(src)?;
                let wireframe = read_bool(src)?;
                let transparency_type = XmacMaterialTransparencyType::try_from(read_u8(src)?)?;
                let layer_count = read_u8(src)?;
                let material_name = read_xmac_str(src, big_endian)?;
                let mut layers = Vec::with_capacity(layer_count as usize);
                for _idx in 0..layer_count {
                    let amount = read_f32_endian(src, big_endian)?;
                    let u_offset = read_f32_endian(src, big_endian)?;
                    let v_offset = read_f32_endian(src, big_endian)?;
                    let u_tiling = read_f32_endian(src, big_endian)?;
                    let v_tiling = read_f32_endian(src, big_endian)?;
                    let rotation_rads = read_f32_endian(src, big_endian)?;
                    let material_id = read_u16_endian(src, big_endian)?;

                    let layer_type = XmacMaterialLayerType::try_from(read_u8(src)?)?;
                    let blend_mode = XmacLayerBlendMode::try_from(read_u8(src)?)?;

                    let texture = read_xmac_str(src, big_endian)?;

                    layers.push(XmacStandardMaterialLayer {
                        texture,
                        ty: layer_type,
                        amount,
                        u_offset,
                        v_offset,
                        u_tiling,
                        v_tiling,
                        rotation_rads,
                        blend_mode,
                        material_id,
                    });
                }
                Ok(Some(Self {
                    name: material_name,
                    layers,

                    ambient_color,
                    diffuse_color,
                    specular_color,
                    emissive_color,
                    shine,
                    shine_strength,
                    opacity,
                    refraction_index,
                    double_sided,
                    wireframe,
                    transparency_type,
                }))
            }
            ver => {
                println!(
                    "Unknown XMAC materials version {ver}@{:x}, skipping",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}

impl XmacMesh {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading MESH chunk...");
        match chunk_meta.version {
            1 => {
                let node_id = read_u32_endian(src, big_endian)?;
                let orig_verts_count = read_u32_endian(src, big_endian)?;
                let total_vertices_count = read_u32_endian(src, big_endian)?;
                let total_indices_count = read_u32_endian(src, big_endian)?;

                let submesh_count = read_u32_endian(src, big_endian)?;
                let layer_count = read_u32_endian(src, big_endian)?;
                // let is_collision_mesh = read_bool(src)?;
                // let is_triangle_mesh = read_bool(src)?;
                let unknown = read_u32_endian(src, big_endian)?;

                let mut layers = Vec::with_capacity(layer_count as usize);
                for _layer_idx in 0..layer_count {
                    layers.push(XmacMeshAttribLayer::load(
                        src,
                        big_endian,
                        multiply_order,
                        total_vertices_count,
                    )?);
                }

                let mut submeshes = Vec::with_capacity(submesh_count as usize);
                for _mesh_idx in 0..submesh_count {
                    submeshes.push(XmacMeshSubmesh::load(src, big_endian)?);
                }

                Ok(Some(Self {
                    vertex_attribute_layers: layers,
                    submeshes,
                    node_id,
                    unknown,
                }))
            }

            ver => {
                println!(
                    "Unknown XMAC mesh version {ver}@{:x}, skipping",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}

impl XmacMeshAttribLayer {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        multiply_order: bool,
        vertices_count: u32,
    ) -> Result<Self> {
        let layer_type = read_u32_endian(src, big_endian)?;
        let layer_type = XmacMeshAttribLayerType::try_from(layer_type)?;
        let attrib_size = read_u32_endian(src, big_endian)?;
        let expected_attrib_size = match &layer_type {
            XmacMeshAttribLayerType::Positions => 3 * 4,
            XmacMeshAttribLayerType::Normals => 3 * 4,
            XmacMeshAttribLayerType::Tangents => 4 * 4,
            XmacMeshAttribLayerType::UvCoords => 2 * 4,
            XmacMeshAttribLayerType::Colors32 => 4,
            XmacMeshAttribLayerType::OriginalVertexNumbers => 4,
            XmacMeshAttribLayerType::Colors128 => 4 * 4,
            XmacMeshAttribLayerType::BiTangents => 3 * 4,
            XmacMeshAttribLayerType::ClothData => 4,
        };
        if attrib_size != expected_attrib_size {
            return Err(Error::InvalidStructure(format!("Attribute size mismatch - {layer_type:?} should have {expected_attrib_size}, found {attrib_size}!")));
        }
        let unknown1 = read_u32_endian(src, big_endian)?;

        let attribs = match &layer_type {
            XmacMeshAttribLayerType::Positions => XmacMeshAttrib::Positions(
                XmacMeshAttrib::load_vector3(src, big_endian, vertices_count)?,
            ),
            XmacMeshAttribLayerType::Normals => XmacMeshAttrib::Normals(
                XmacMeshAttrib::load_vector3(src, big_endian, vertices_count)?,
            ),
            XmacMeshAttribLayerType::Tangents => XmacMeshAttrib::Tangents(
                XmacMeshAttrib::load_vector4(src, big_endian, vertices_count)?,
            ),
            XmacMeshAttribLayerType::UvCoords => XmacMeshAttrib::UvCoords(
                XmacMeshAttrib::load_vector2(src, big_endian, vertices_count)?,
            ),
            XmacMeshAttribLayerType::Colors32 => {
                XmacMeshAttrib::Colors32(XmacMeshAttrib::load_u32(src, big_endian, vertices_count)?)
            }
            XmacMeshAttribLayerType::OriginalVertexNumbers => {
                XmacMeshAttrib::OriginalVertexNumbers(XmacMeshAttrib::load_u32(
                    src,
                    big_endian,
                    vertices_count,
                )?)
            }
            XmacMeshAttribLayerType::Colors128 => XmacMeshAttrib::Colors128(
                XmacMeshAttrib::load_vector4(src, big_endian, vertices_count)?,
            ),
            XmacMeshAttribLayerType::BiTangents => XmacMeshAttrib::BiTangents(
                XmacMeshAttrib::load_vector3(src, big_endian, vertices_count)?,
            ),
            XmacMeshAttribLayerType::ClothData => XmacMeshAttrib::ClothData(
                XmacMeshAttrib::load_u32(src, big_endian, vertices_count)?,
            ),
        };

        Ok(Self { attribs, unknown1 })
    }
}

impl XmacMeshAttrib {
    fn load_vector2<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<Vector2>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(Vector2::load_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
    fn load_vector3<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<Vector3>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(Vector3::load_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
    fn load_vector4<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<Vector4>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(Vector4::load_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
    fn load_u32<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<u32>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(read_u32_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
}

impl XmacMeshSubmesh {
    fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let indices_count = read_u32_endian(src, big_endian)?;
        let vertices_count = read_u32_endian(src, big_endian)?;
        let material_idx = read_u32_endian(src, big_endian)?;
        let bones_count = read_u32_endian(src, big_endian)?;

        let mut indices = Vec::with_capacity(indices_count as usize);
        for _idx in 0..indices_count {
            indices.push(read_u32_endian(src, big_endian)?);
        }
        let mut bones = Vec::with_capacity(bones_count as usize);
        for _idx in 0..bones_count {
            bones.push(read_u32_endian(src, big_endian)?);
        }
        Ok(Self {
            indices,
            bones,
            vertices_count,
            material_idx,
        })
    }
}

/// XMAC Strings store their length in (endianness-affected) u32
fn read_xmac_str<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<String> {
    let len = read_u32_endian(src, big_endian)? as usize;
    if len > 255 {
        return Err(Error::InvalidStructure(format!(
            "String @{} is supposedly {len} bytes",
            src.stream_position()?
        )));
    }
    let mut str_buf = vec![0; len];
    src.read_exact(&mut str_buf)?;

    // TODO: Check if Xmac files contain UTF8?
    if let Some(string) =
        encoding_rs::WINDOWS_1252.decode_without_bom_handling_and_without_replacement(&str_buf)
    {
        Ok(string.to_string())
    } else {
        Err(Error::InvalidString(format!("{:x?}", str_buf)))
    }
}
