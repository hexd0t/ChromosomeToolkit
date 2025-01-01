use std::ops::Index;
use std::ops::IndexMut;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::super::read_xmac_str;

use super::XmacChunkMeta;
use super::XmacChunkType;
use crate::archive::ArchiveReadTarget;
use crate::archive::ArchiveWriteTarget;
use crate::binimport::BinImport;
use crate::error::*;
use crate::helpers::*;
use crate::types::{Mat4, Quat, Vec3, Vec4};
use crate::xmac::write_xmac_str;

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct XmacNodeId(pub u32);

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacNodes {
    pub nodes: Vec<XmacNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacNode {
    pub name: String,
    pub rotation: Quat,
    /// [0, 0, 0, 1] for nearly everything but root- and slot-nodes,
    /// so probably something about lining multiple meshes up
    pub unknown1: Vec4,
    pub local_pos: Vec3,
    pub local_scale: Vec3,
    /// always [-1, -1, -1] for R1
    pub unknown2: Vec3,
    /// always -1 for R1
    pub unknown3: i32,
    /// always -1 for R1
    pub unknown4: i32,
    pub parent_idx: Option<usize>,
    pub child_count: u32,
    pub flags: XmacNodeFlags,
    pub unknown5: [u8; 3],
    #[serde(deserialize_with = "deserialize_mat4_with_nan")]
    pub oriented_bounding_box: glam::Mat4,
    /// usually 1.0
    pub unknown6: f32,
}

fn deserialize_mat4_with_nan<'de, D: serde::Deserializer<'de>>(
    des: D,
) -> std::result::Result<glam::Mat4, D::Error> {
    let optionals = <[Option<f32>; 16]>::deserialize(des)?;
    let mut nans = [0.0; 16];
    for i in 0..16 {
        nans[i] = optionals[i].unwrap_or(f32::NAN)
    }
    Ok(Mat4::from_cols_array(&nans))
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

        const Unknown08 = 0x8;
        const Unknown10 = 0x10;
        const Unknown20 = 0x20;
        const Unknown40 = 0x40;
        const Unknown80 = 0x80;
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
                    let node = XmacNode::load(src, big_endian)?;

                    nodes.push(node);
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
    pub fn save<W: ArchiveWriteTarget>(
        &self,
        dst: &mut W,
        big_endian: bool,
    ) -> Result<XmacChunkMeta> {
        println!("Saving NODES chunk...");
        write_u32_endian(dst, self.nodes.len() as u32, big_endian)?;
        write_u32_endian(
            dst,
            self.nodes.iter().filter(|n| n.parent_idx.is_none()).count() as u32,
            big_endian,
        )?;
        let mut written = 8;
        for node in &self.nodes {
            written += node.save(dst, big_endian)?;
        }
        Ok(XmacChunkMeta {
            type_id: XmacChunkType::Nodes.into(),
            size: written as u32,
            version: 1,
        })
    }
}

impl XmacNode {
    fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<XmacNode> {
        let rotation = Quat::load_endian(src, big_endian)?;
        let unknown1 = Vec4::load_endian(src, big_endian)?;
        let local_pos = Vec3::load_endian(src, big_endian)?;
        let local_scale = Vec3::load_endian(src, big_endian)?;
        let unknown2 = Vec3::load_endian(src, big_endian)?;
        let unknown3 = read_i32_endian(src, big_endian)?;
        let unknown4 = read_i32_endian(src, big_endian)?;
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
        let mut unknown5 = [0; 3];
        src.read_exact(&mut unknown5)?;
        let oriented_bounding_box = Mat4::load(src)?;
        let unknown6 = read_f32_endian(src, big_endian)?;
        let node_name = read_xmac_str(src, big_endian)?;
        let node = XmacNode {
            name: node_name,
            rotation,
            unknown1,
            local_pos,
            local_scale,
            unknown2,
            unknown3,
            unknown4,
            parent_idx,
            child_count,
            flags,
            unknown5,
            oriented_bounding_box,
            unknown6,
        };
        Ok(node)
    }

    fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        self.rotation.save_endian(dst, big_endian)?;
        self.unknown1.save_endian(dst, big_endian)?;
        self.local_pos.save_endian(dst, big_endian)?;
        self.local_scale.save_endian(dst, big_endian)?;
        self.unknown2.save_endian(dst, big_endian)?;
        let mut written = 16 + 16 + 12 + 12 + 12;

        write_i32_endian(dst, self.unknown3, big_endian)?;
        write_i32_endian(dst, self.unknown4, big_endian)?;
        write_i32_endian(
            dst,
            self.parent_idx.map(|i| i as i32).unwrap_or(-1),
            big_endian,
        )?;
        write_u32_endian(dst, self.child_count, big_endian)?;
        write_u8(dst, self.flags.bits())?;
        dst.write_all(&self.unknown5)?;
        written += 4 + 4 + 4 + 4 + 1 + 3;

        self.oriented_bounding_box.save_endian(dst, big_endian)?;
        write_f32_endian(dst, self.unknown6, big_endian)?;
        written += 4 * 16 + 4;
        written += write_xmac_str(dst, &self.name, big_endian)?;
        Ok(written)
    }

    pub fn new(name: String, local_trans: Mat4, parent_idx: Option<usize>) -> Self {
        let (mut local_scale, rotation, local_pos) = local_trans.to_scale_rotation_translation();
        local_scale = 1.0 / local_scale; //Scale is inverted for some reason
        Self {
            name,
            rotation,
            unknown1: Vec4::new(0.0, 0.0, 0.0, 1.0),
            local_pos,
            local_scale,
            unknown2: Vec3::new(-1.0, -1.0, -1.0),
            unknown3: -1,
            unknown4: -1,
            parent_idx,
            child_count: 0,
            flags: XmacNodeFlags::Critical | XmacNodeFlags::IncludeInBoundsCalc,
            unknown5: [0, 0, 0],
            unknown6: 0.0,
            oriented_bounding_box: Mat4::IDENTITY,
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
        self.nodes.iter()
    }
}
