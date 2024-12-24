//! Resources on the XMAC format:
//!  - O3DE (fomerly Amazon Lumberyard) has an importer for XMAC, although a newer version
//!    at https://github.com/o3de/o3de/tree/development/Gems/EMotionFX/Code/EMotionFX/Source
//!    older commits contain more 'legacy' RemoteFX code
//!    (since Amazon bought EmotionFX & employs their devs, this can be considered 'ground truth')
//!    License: Apache 2 or MIT
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
    timestamp: DateTime,
    props: Vec<Property>,
    chunks: Vec<XmacChunk>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum XmacChunk {
    Info(XmacInfo),
    Nodes(XmacNodes),
    Materials(XmacMaterials),
    Unknown(XmacUnknownChunk),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacUnknownChunk {
    type_id: u32,
    version: u32,
    #[serde(with = "crate::helpers::ser_hex")]
    data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacInfo {
    /// the number of level of details
    lod_count: u32,
    // TODO: The version used by R1 carries only one of these:
    /// the node number of the trajectory node used for motion extraction
    motion_extraction_node_index: i32,
    /// the retargeting root node index, most likely pointing to the hip or pelvis or MCORE_INVALIDINDEX32 when not set
    //retarget_root_node_index: i32,
    /// supposedly contains unit_type (feet, cm, m, ...) and exporter version, but none of those are used
    unknown1: u32,
    unknown2: u32,
    /// source application (e.g. "3ds Max 2011", "Maya 2011")
    source_application: String,
    /// original filename of the 3dsMax/Maya file
    orig_filename: String,
    /// compilation date of the exporter
    exporter_date: String,
    /// the name of the actor
    actor_name: String,
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
pub struct XmacMaterials {
    materials: Vec<XmacMaterial>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMaterial {
    name: String,
    maps: Vec<XmacMap>,
    unknown2: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMap {
    texture: String,
    map_type: XmacMapType,
    unknown3: [f32; 6],
    unknown4: u16,
    unknown5: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum XmacMapType {
    Diffuse,
    Specular,
    Normal,
}

#[repr(u32)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]

pub enum XmacChunkType {
    Node = 0,
    Limit = 6,
    Info = 7,
    StdProgMorphTarget = 9,
    NodeGroups = 10,
    Nodes = 11,
    StdPMorphTargets = 12,
    /// Chunk 13 is originally just MaterialsCount,
    /// with Chunk 3 containing the Materials -
    /// but it's Chunk size is broken and it's easier to handle both at once
    Materials = 13,
    NodeMotionSources = 14,
    AttachmentNodes = 15,
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
            chunks.push(XmacChunk::load(src, big_endian, multiply_order)?);
        }

        Ok(chunks)
    }
}

impl XmacChunk {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        multiply_order: bool,
    ) -> Result<Self> {
        let chunk_type_id = read_u32_endian(src, big_endian)?;
        let chunk_size = read_u32_endian(src, big_endian)?;
        let chunk_version = read_u32_endian(src, big_endian)?;
        let chunk_start = src.stream_position()?;
        let chunk_end = chunk_start + chunk_size as u64;
        let result = if let Ok(chunk_type) = chunk_type_id.try_into() {
            match chunk_type {
                XmacChunkType::Info => {
                    XmacInfo::load(src, big_endian, multiply_order, chunk_size, chunk_version)?
                        .map(XmacChunk::Info)
                        .unwrap_or_else(|| {
                            XmacChunk::Unknown(
                                XmacUnknownChunk::load(
                                    src,
                                    chunk_type_id,
                                    chunk_size,
                                    chunk_version,
                                )
                                .unwrap(),
                            )
                        })
                }
                XmacChunkType::Nodes => {
                    XmacNodes::load(src, big_endian, multiply_order, chunk_size, chunk_version)?
                        .map(XmacChunk::Nodes)
                        .unwrap_or_else(|| {
                            XmacChunk::Unknown(
                                XmacUnknownChunk::load(
                                    src,
                                    chunk_type_id,
                                    chunk_size,
                                    chunk_version,
                                )
                                .unwrap(),
                            )
                        })
                }
                XmacChunkType::Materials => {
                    XmacMaterials::load(src, big_endian, multiply_order, chunk_size, chunk_version)?
                        .map(XmacChunk::Materials)
                        .unwrap_or_else(|| {
                            XmacChunk::Unknown(
                                XmacUnknownChunk::load(
                                    src,
                                    chunk_type_id,
                                    chunk_size,
                                    chunk_version,
                                )
                                .unwrap(),
                            )
                        })
                }
                _ => {
                    println!(
                        "Unimplemented XMAC chunk {chunk_type:?}.{chunk_version}@{:x}",
                        src.stream_position()?
                    );
                    XmacChunk::Unknown(XmacUnknownChunk::load(
                        src,
                        chunk_type_id,
                        chunk_size,
                        chunk_version,
                    )?)
                }
            }
        } else {
            println!(
                "Unknown XMAC chunk id {chunk_type_id}.{chunk_version}@{:x}, skipping",
                src.stream_position()?
            );
            XmacChunk::Unknown(XmacUnknownChunk::load(
                src,
                chunk_type_id,
                chunk_size,
                chunk_version,
            )?)
        };
        if src.stream_position()? != chunk_end && chunk_type_id != 13 {
            println!(
                "Chunk did not read exactly it's size, 0x{:x} vs expected 0x{:x}-0x{:x}!",
                src.stream_position()?,
                chunk_start,
                chunk_end
            );
            //src.seek(std::io::SeekFrom::Start(chunk_end))?;
        }
        Ok(result)
    }
}

impl XmacUnknownChunk {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        chunk_type_id: u32,
        chunk_size: u32,
        chunk_version: u32,
    ) -> Result<Self> {
        let mut data = vec![0; chunk_size as usize];
        src.read_exact(&mut data)?;
        Ok(Self {
            type_id: chunk_type_id,
            version: chunk_version,
            data,
        })
    }
}

impl XmacInfo {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_size: u32,
        chunk_version: u32,
    ) -> Result<Option<Self>> {
        println!("Loading INFO chunk...");
        match chunk_version {
            2 => Ok(Some(XmacInfo {
                lod_count: read_u32_endian(src, big_endian)?,
                motion_extraction_node_index: read_i32_endian(src, big_endian)?,
                //retarget_root_node_index: read_i32_endian(src, big_endian)?,
                unknown1: read_u32_endian(src, big_endian)?,
                unknown2: read_u32_endian(src, big_endian)?,
                source_application: read_xmac_str(src, big_endian)?,
                orig_filename: read_xmac_str(src, big_endian)?,
                exporter_date: read_xmac_str(src, big_endian)?,
                actor_name: read_xmac_str(src, big_endian)?,
            })),
            _ => {
                println!(
                    "Unknown XMAC info version {chunk_version}@{:x}, skipping",
                    src.stream_position()?
                );
                src.seek_relative(chunk_size as i64)?;
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
        chunk_size: u32,
        chunk_version: u32,
    ) -> Result<Option<Self>> {
        println!("Loading NODES chunk...");
        match chunk_version {
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
                    let flags = XmacNodeFlags::from_bits(flags).ok_or(Error::EnumUnparseable(
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
            _ => {
                println!(
                    "Unknown XMAC nodes version {chunk_version}@{:x}, skipping",
                    src.stream_position()?
                );
                src.seek_relative(chunk_size as i64)?;
                Ok(None)
            }
        }
    }
}

impl XmacMaterials {
    fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_size: u32,
        chunk_version: u32,
    ) -> Result<Option<Self>> {
        println!("Loading MATERIALS chunk...");
        match chunk_version {
            1 => {
                let material_count = read_u32_endian(src, big_endian)? as usize;
                let root_count = read_u32_endian(src, big_endian)? as usize;
                if material_count != root_count {
                    return Err(Error::InvalidStructure(format!(
                        "Cascaded Materials are not supported"
                    )));
                }
                let materials_chunk_id = read_u32_endian(src, big_endian)?;
                assert_eq!(materials_chunk_id, 3);
                let mut materials = Vec::with_capacity(material_count);
                for _idx in 0..material_count {
                    let mut unknown2 = [0u8; 95];
                    src.read_exact(&mut unknown2)?;
                    let map_count = read_u8(src)? as usize;
                    let material_name = read_xmac_str(src, big_endian)?;
                    let mut maps = Vec::with_capacity(map_count);
                    for _idx in 0..map_count {
                        let mut unknown3 = [0f32; 6];
                        for f in unknown3.iter_mut() {
                            *f = read_f32_endian(src, big_endian)?;
                        }
                        let unknown4 = read_u16_endian(src, big_endian)?;

                        let map_type = match read_u8(src)? {
                            2 => XmacMapType::Diffuse,
                            3 => XmacMapType::Specular,
                            5 => XmacMapType::Normal,
                            type_id => {
                                return Err(Error::EnumUnparseable(format!(
                                    "Unknown XmacMapType: {type_id}"
                                )))
                            }
                        };
                        let unknown5 = read_u8(src)?;

                        let texture = read_xmac_str(src, big_endian)?;

                        maps.push(XmacMap {
                            texture,
                            map_type,
                            unknown3,
                            unknown4,
                            unknown5,
                        });
                    }
                    materials.push(XmacMaterial {
                        name: material_name,
                        maps,
                        unknown2: Vec::from(unknown2),
                    });

                    println!("{:?}\n---", materials[materials.len() - 1]);
                }

                Ok(Some(XmacMaterials { materials }))
            }
            _ => {
                println!(
                    "Unknown XMAC materials version {chunk_version}@{:x}, skipping",
                    src.stream_position()?
                );
                src.seek_relative(chunk_size as i64)?;
                Ok(None)
            }
        }
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

    // TODO: Check if xmacs contain UTF8?
    if let Some(string) =
        encoding_rs::WINDOWS_1252.decode_without_bom_handling_and_without_replacement(&str_buf)
    {
        Ok(string.to_string())
    } else {
        Err(Error::InvalidString(format!("{:x?}", str_buf)))
    }
}
