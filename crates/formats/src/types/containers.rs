use serde::{Deserialize, Serialize};

use crate::archive::ArchiveSerializable;
use crate::helpers::*;

#[derive(Debug, Serialize)]
/// bTRefPtrArray
pub struct RefPtrArray<T: ArchiveSerializable>(pub Vec<T>);

impl<T: ArchiveSerializable> ArchiveSerializable for RefPtrArray<T> {
    fn load<R: super::ArchiveReadTarget>(src: &mut R) -> super::Result<Self> {
        let unknown1 = read_bool(src)?;
        if !unknown1 {
            return Ok(Self(Vec::new()));
        }
        println!("Count@{:x}", src.stream_position()?);
        let count = read_u32(src)? as usize;
        let mut result = Vec::with_capacity(count);
        println!(".{count:x}");
        for _idx in 0..count {
            result.push(T::load(src)?);
            print!(":");
        }
        println!("-");
        Ok(Self(result))
    }

    fn save<W: super::ArchiveWriteTarget>(&self, dst: &mut W) -> super::Result<()> {
        write_bool(dst, true)?;
        write_u32(dst, self.0.len() as u32)?;
        for item in &self.0 {
            item.save(dst)?;
        }
        Ok(())
    }
}

impl<'de, T: ArchiveSerializable> Deserialize<'de> for RefPtrArray<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Vec::<T>::deserialize(deserializer)?))
    }
}
