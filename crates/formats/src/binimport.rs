use std::io::{Read, Result, Write};

use crate::helpers::*;

pub trait BinImport {
    fn load<R: Read>(src: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_endian(src, false)
    }
    fn load_endian<R: Read>(src: &mut R, big_endian: bool) -> Result<Self>
    where
        Self: Sized;
    fn save<W: Write>(&self, dst: &mut W) -> Result<()> {
        self.save_endian(dst, false)
    }
    fn save_endian<W: Write>(&self, dst: &mut W, big_endian: bool) -> Result<()>;
}

impl BinImport for glam::Vec2 {
    fn load_endian<R: Read>(src: &mut R, big_endian: bool) -> Result<Self> {
        let data = [
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
        ];
        Ok(Self::from_array(data))
    }
    fn save_endian<W: Write>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        let data = self.to_array();
        for d in data {
            write_f32_endian(dst, d, big_endian)?;
        }
        Ok(())
    }
}
impl BinImport for glam::Vec3 {
    fn load_endian<R: Read>(src: &mut R, big_endian: bool) -> Result<Self> {
        let data = [
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
        ];
        Ok(Self::from_array(data))
    }
    fn save_endian<W: Write>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        let data = self.to_array();
        for d in data {
            write_f32_endian(dst, d, big_endian)?;
        }
        Ok(())
    }
}
impl BinImport for glam::Vec4 {
    fn load_endian<R: Read>(src: &mut R, big_endian: bool) -> Result<Self> {
        let data = [
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
        ];
        Ok(Self::from_array(data))
    }
    fn save_endian<W: Write>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        let data = self.to_array();
        for d in data {
            write_f32_endian(dst, d, big_endian)?;
        }
        Ok(())
    }
}
impl BinImport for glam::Quat {
    /// This reads in XYZW order!
    fn load_endian<R: Read>(src: &mut R, big_endian: bool) -> Result<Self> {
        let data = [
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
            read_f32_endian(src, big_endian)?,
        ];
        Ok(Self::from_array(data))
    }
    /// This stores in XYZW order!
    fn save_endian<W: Write>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        let data = self.to_array();
        for d in data {
            write_f32_endian(dst, d, big_endian)?;
        }
        Ok(())
    }
}
impl BinImport for glam::Mat4 {
    fn load_endian<R: Read>(src: &mut R, big_endian: bool) -> Result<Self> {
        Ok(Self {
            x_axis: glam::Vec4::load_endian(src, big_endian)?,
            y_axis: glam::Vec4::load_endian(src, big_endian)?,
            z_axis: glam::Vec4::load_endian(src, big_endian)?,
            w_axis: glam::Vec4::load_endian(src, big_endian)?,
        })
    }
    fn save_endian<W: Write>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        self.x_axis.save_endian(dst, big_endian)?;
        self.y_axis.save_endian(dst, big_endian)?;
        self.z_axis.save_endian(dst, big_endian)?;
        self.w_axis.save_endian(dst, big_endian)?;
        Ok(())
    }
}
