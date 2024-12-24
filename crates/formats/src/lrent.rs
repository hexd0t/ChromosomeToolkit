use serde::{Deserialize, Serialize};

use super::helpers::*;
use crate::{archive::PakFile, error::*, types::object::AccessorPropertyObject};

#[derive(Debug, Deserialize, Serialize)]
pub struct LrentFile {
    pub root: AccessorPropertyObject,
}

impl LrentFile {
    pub fn load(mut arch: PakFile) -> Result<Self> {
        let magic = read_u32(&mut arch)?;
        assert_eq!(magic, 0xd0defade);
        let lrent_file = AccessorPropertyObject::load(&mut arch)?;
        println!(
            "finished at: {:x} / {:x}",
            arch.current_read_idx + 14,
            arch.data.len() + 14
        );

        Ok(Self { root: lrent_file })
    }

    pub fn save(&self, arch: &mut PakFile) -> Result<()> {
        let magic = 0xd0defade;
        write_u32(arch, magic)?;
        self.root.save(arch)?;
        Ok(())
    }
}
