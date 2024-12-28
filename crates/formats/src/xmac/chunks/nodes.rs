use std::ops::Index;
use std::ops::IndexMut;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::super::read_xmac_str;

use super::XmacChunkMeta;
use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;
use crate::types::{Matrix, Quaternion, Vector2, Vector3, Vector4};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct XmacNodeId(pub u32);

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacNodes {
    pub nodes: Vec<XmacNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacNode {
    pub name: String,
    pub rotation: Quaternion,
    pub unknown1: Vector4,
    pub local_pos: Vector3,
    pub local_scale: Vector3,
    pub unknown2: Vector3,
    pub unknown3: Vector2,
    pub parent_idx: Option<usize>,
    pub child_count: u32,
    pub flags: XmacNodeFlags,
    pub unknown4: [u8; 3],
    pub unknown5: f32,
    pub oriented_bounding_box: Matrix,
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

impl XmacNodes {
    pub fn load<R: ArchiveReadTarget>(
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
                    let flags = XmacNodeFlags::from_bits(flags).ok_or_else(|| {
                        Error::EnumUnparsable(format!(
                            "Parsing XmacNodeFlags failed, invalid value {flags:02x}@{:x}",
                            src.stream_position().unwrap_or_default()
                        ))
                    })?;

                    let mut unknown4 = [0; 3];
                    src.read_exact(&mut unknown4)?;

                    // might be the other way around:
                    let unknown5 = read_f32_endian(src, big_endian)?;
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
                        unknown5,
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

impl Index<XmacNodeId> for XmacNodes {
    type Output = XmacNode;

    fn index(&self, index: XmacNodeId) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}
impl IndexMut<XmacNodeId> for XmacNodes {
    fn index_mut(&mut self, index: XmacNodeId) -> &mut Self::Output {
        &mut self.nodes[index.0 as usize]
    }
}
impl IntoIterator for XmacNodes {
    type Item = XmacNode;

    type IntoIter = <Vec<XmacNode> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}
impl<'a> IntoIterator for &'a XmacNodes {
    type Item = &'a XmacNode;

    type IntoIter = <&'a Vec<XmacNode> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.nodes).into_iter()
    }
}
