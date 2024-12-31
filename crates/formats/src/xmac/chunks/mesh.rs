use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use super::nodes::XmacNodeId;
use super::XmacChunkMeta;
use super::XmacChunkType;

use crate::archive::ArchiveReadTarget;
use crate::archive::ArchiveWriteTarget;
use crate::binimport::BinImport;
use crate::error::*;
use crate::helpers::*;
use crate::types::Vec2;
use crate::types::Vec3;
use crate::types::Vec4;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMesh {
    pub vertex_attribute_layers: Vec<XmacMeshAttribLayer>,
    pub submeshes: Vec<XmacMeshSubmesh>,

    pub node_id: XmacNodeId,
    pub orig_verts_count: u32,
    pub is_collision_mesh: bool,
    pub unknown1: [u8; 3],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMeshAttribLayer {
    pub attribs: XmacMeshAttrib,
    pub unknown1: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum XmacMeshAttrib {
    Positions(Vec<Vec3>),
    Normals(Vec<Vec3>),
    Tangents(Vec<Vec4>),
    UvCoords(Vec<Vec2>),
    /// Contains a 4-byte RGBA value
    Colors32(Vec<u32>),
    OriginalVertexNumbers(Vec<u32>),
    /// Contains 4 f32 color entries (RGBA)
    Colors128(Vec<Vec4>),
    BiTangents(Vec<Vec3>),
    ClothData(Vec<u32>),
}

#[repr(u32)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacMeshAttribLayerType {
    /// Contains a Vec3
    Positions = 0,
    /// Contains a Vec3
    Normals = 1,
    /// Contains a Vec4
    Tangents = 2,
    /// Contains a Vec2
    UvCoords = 3,
    /// Contains a 4-byte RGBA value
    Colors32 = 4,
    /// Contains a u32
    OriginalVertexNumbers = 5,
    /// Contains 4 f32 color entries (RGBA)
    Colors128 = 6,
    /// Contains a Vec3
    BiTangents = 7,
    /// Contains a u32
    ClothData = 8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMeshSubmesh {
    pub indices: Vec<u32>,
    pub bones: Vec<u32>,

    pub vertices_count: u32,
    pub material_idx: u32,
}

impl XmacMesh {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading MESH chunk...");
        match chunk_meta.version {
            1 => {
                let node_id = XmacNodeId(read_u32_endian(src, big_endian)?);
                let orig_verts_count = read_u32_endian(src, big_endian)?;
                let total_vertices_count = read_u32_endian(src, big_endian)?;
                let total_indices_count = read_u32_endian(src, big_endian)?;

                let submesh_count = read_u32_endian(src, big_endian)?;
                let layer_count = read_u32_endian(src, big_endian)?;
                let is_collision_mesh = read_bool(src)?;

                let mut unknown1 = [0u8; 3];
                src.read_exact(&mut unknown1)?;

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
                let mut loaded_indices = 0;
                let mut used_vertices = 0;
                for _mesh_idx in 0..submesh_count {
                    let submesh = XmacMeshSubmesh::load(src, big_endian)?;
                    loaded_indices += submesh.indices.len();
                    used_vertices += submesh.vertices_count;
                    submeshes.push(submesh);
                }
                assert_eq!(loaded_indices, total_indices_count as usize);
                assert_eq!(used_vertices, total_vertices_count);

                Ok(Some(Self {
                    vertex_attribute_layers: layers,
                    submeshes,
                    node_id,
                    orig_verts_count,
                    unknown1,
                    is_collision_mesh,
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

    pub fn save<W: ArchiveWriteTarget>(
        &self,
        dst: &mut W,
        big_endian: bool,
    ) -> Result<XmacChunkMeta> {
        write_u32_endian(dst, self.node_id.0, big_endian)?;
        write_u32_endian(dst, self.orig_verts_count, big_endian)?;
        let total_verts = self.get_position_attrib().unwrap().len();
        write_u32_endian(dst, total_verts as u32, big_endian)?;
        let total_indices: usize = self.submeshes.iter().map(|s| s.indices.len()).sum();
        write_u32_endian(dst, total_indices as u32, big_endian)?;

        write_u32_endian(dst, self.submeshes.len() as u32, big_endian)?;
        write_u32_endian(dst, self.vertex_attribute_layers.len() as u32, big_endian)?;
        write_bool(dst, self.is_collision_mesh)?;
        // let is_triangle_mesh = read_bool(src)?;
        dst.write_all(&self.unknown1)?;
        let mut written = 4 + 4 + 4 + 4 + 4 + 4 + 1 + 3;
        for layer in &self.vertex_attribute_layers {
            written += layer.save(dst, big_endian)?;
        }
        for submesh in &self.submeshes {
            written += submesh.save(dst, big_endian)?;
        }

        Ok(XmacChunkMeta {
            type_id: XmacChunkType::Mesh.into(),
            size: written as u32,
            version: 1,
        })
    }

    pub fn get_position_attrib(&self) -> Option<&Vec<Vec3>> {
        fn get_position_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vec3>> {
            if let XmacMeshAttrib::Positions(val) = &attrib.attribs {
                Some(val)
            } else {
                None
            }
        }
        self.vertex_attribute_layers
            .iter()
            .find_map(get_position_attrib)
    }
    pub fn get_normal_attrib(&self) -> Option<&Vec<Vec3>> {
        fn get_normal_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vec3>> {
            if let XmacMeshAttrib::Normals(val) = &attrib.attribs {
                Some(val)
            } else {
                None
            }
        }
        self.vertex_attribute_layers
            .iter()
            .find_map(get_normal_attrib)
    }
    pub fn get_tangent_attrib(&self) -> Option<&Vec<Vec4>> {
        fn get_tangent_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vec4>> {
            if let XmacMeshAttrib::Tangents(val) = &attrib.attribs {
                Some(val)
            } else {
                None
            }
        }
        self.vertex_attribute_layers
            .iter()
            .find_map(get_tangent_attrib)
    }
    pub fn get_uv_attrib(&self) -> Option<&Vec<Vec2>> {
        fn get_uv_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<Vec2>> {
            if let XmacMeshAttrib::UvCoords(val) = &attrib.attribs {
                Some(val)
            } else {
                None
            }
        }
        self.vertex_attribute_layers.iter().find_map(get_uv_attrib)
    }

    pub fn get_orig_vert(&self) -> Option<&Vec<u32>> {
        fn get_orig_vert_attrib(attrib: &XmacMeshAttribLayer) -> Option<&Vec<u32>> {
            if let XmacMeshAttrib::OriginalVertexNumbers(val) = &attrib.attribs {
                Some(val)
            } else {
                None
            }
        }

        self.vertex_attribute_layers
            .iter()
            .find_map(get_orig_vert_attrib)
    }
}

impl XmacMeshAttribLayer {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        vertices_count: u32,
    ) -> Result<Self> {
        let layer_type = read_u32_endian(src, big_endian)?;
        let layer_type = XmacMeshAttribLayerType::try_from(layer_type)?;
        let attrib_size = read_u32_endian(src, big_endian)?;
        let expected_attrib_size = layer_type.get_expected_attrib_size();
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

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        let type_id = match &self.attribs {
            XmacMeshAttrib::Positions(_) => XmacMeshAttribLayerType::Positions,
            XmacMeshAttrib::Normals(_) => XmacMeshAttribLayerType::Normals,
            XmacMeshAttrib::Tangents(_) => XmacMeshAttribLayerType::Tangents,
            XmacMeshAttrib::UvCoords(_) => XmacMeshAttribLayerType::UvCoords,
            XmacMeshAttrib::Colors32(_) => XmacMeshAttribLayerType::Colors32,
            XmacMeshAttrib::OriginalVertexNumbers(_) => {
                XmacMeshAttribLayerType::OriginalVertexNumbers
            }
            XmacMeshAttrib::Colors128(_) => XmacMeshAttribLayerType::Colors128,
            XmacMeshAttrib::BiTangents(_) => XmacMeshAttribLayerType::BiTangents,
            XmacMeshAttrib::ClothData(_) => XmacMeshAttribLayerType::ClothData,
        };
        write_u32_endian(dst, type_id.into(), big_endian)?;
        write_u32_endian(dst, type_id.get_expected_attrib_size(), big_endian)?;
        write_u32_endian(dst, self.unknown1, big_endian)?;
        let mut written = 4 + 4 + 4;
        written += match &self.attribs {
            XmacMeshAttrib::Tangents(data) | XmacMeshAttrib::Colors128(data) => {
                XmacMeshAttrib::save_vector4(dst, data, big_endian)?
            }
            XmacMeshAttrib::BiTangents(data)
            | XmacMeshAttrib::Positions(data)
            | XmacMeshAttrib::Normals(data) => XmacMeshAttrib::save_vector3(dst, data, big_endian)?,
            XmacMeshAttrib::UvCoords(data) => XmacMeshAttrib::save_vector2(dst, data, big_endian)?,
            XmacMeshAttrib::Colors32(data)
            | XmacMeshAttrib::OriginalVertexNumbers(data)
            | XmacMeshAttrib::ClothData(data) => XmacMeshAttrib::save_u32(dst, data, big_endian)?,
        };
        Ok(written)
    }
}

impl XmacMeshAttrib {
    pub fn load_vector2<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<Vec2>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(Vec2::load_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
    pub fn save_vector2<W: ArchiveWriteTarget>(
        dst: &mut W,
        data: &Vec<Vec2>,
        big_endian: bool,
    ) -> Result<usize> {
        let mut written = 0;
        for ver in data {
            ver.save_endian(dst, big_endian)?;
            written += 2 * 4;
        }
        Ok(written)
    }
    pub fn load_vector3<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<Vec3>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(Vec3::load_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
    pub fn save_vector3<W: ArchiveWriteTarget>(
        dst: &mut W,
        data: &Vec<Vec3>,
        big_endian: bool,
    ) -> Result<usize> {
        let mut written = 0;
        for ver in data {
            ver.save_endian(dst, big_endian)?;
            written += 3 * 4;
        }
        Ok(written)
    }
    pub fn load_vector4<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        vertices_count: u32,
    ) -> Result<Vec<Vec4>> {
        let mut attribs = Vec::with_capacity(vertices_count as usize);

        for _ver_idx in 0..vertices_count {
            attribs.push(Vec4::load_endian(src, big_endian)?);
        }
        Ok(attribs)
    }
    pub fn save_vector4<W: ArchiveWriteTarget>(
        dst: &mut W,
        data: &Vec<Vec4>,
        big_endian: bool,
    ) -> Result<usize> {
        let mut written = 0;
        for ver in data {
            ver.save_endian(dst, big_endian)?;
            written += 4 * 4;
        }
        Ok(written)
    }
    pub fn load_u32<R: ArchiveReadTarget>(
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
    pub fn save_u32<W: ArchiveWriteTarget>(
        dst: &mut W,
        data: &Vec<u32>,
        big_endian: bool,
    ) -> Result<usize> {
        let mut written = 0;
        for ver in data {
            write_u32_endian(dst, *ver, big_endian)?;
            written += 4;
        }
        Ok(written)
    }
}

impl XmacMeshSubmesh {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
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
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        write_u32_endian(dst, self.indices.len() as u32, big_endian)?;
        write_u32_endian(dst, self.vertices_count, big_endian)?;
        write_u32_endian(dst, self.material_idx, big_endian)?;
        write_u32_endian(dst, self.bones.len() as u32, big_endian)?;
        let mut written = 4 * 4;
        for idx in &self.indices {
            write_u32_endian(dst, *idx, big_endian)?;
            written += 4;
        }
        for bone in &self.bones {
            write_u32_endian(dst, *bone, big_endian)?;
            written += 4;
        }
        Ok(written)
    }
}

impl XmacMeshAttribLayerType {
    pub fn get_expected_attrib_size(&self) -> u32 {
        match &self {
            XmacMeshAttribLayerType::Positions => 3 * 4,
            XmacMeshAttribLayerType::Normals => 3 * 4,
            XmacMeshAttribLayerType::Tangents => 4 * 4,
            XmacMeshAttribLayerType::UvCoords => 2 * 4,
            XmacMeshAttribLayerType::Colors32 => 4,
            XmacMeshAttribLayerType::OriginalVertexNumbers => 4,
            XmacMeshAttribLayerType::Colors128 => 4 * 4,
            XmacMeshAttribLayerType::BiTangents => 3 * 4,
            XmacMeshAttribLayerType::ClothData => 4,
        }
    }
}
