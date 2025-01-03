use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::helpers::*;
use crate::{archive::PakFile, error::*, types::template::TemplateEntity};

#[derive(Debug, Deserialize, Serialize)]
pub struct TpleFile {
    pub entities: Vec<TemplateEntity>,

    pub parents: Vec<(i32, i32)>,
}

impl TpleFile {
    pub fn load(mut arch: PakFile) -> Result<Self> {
        let mut magic = [0u8; 8];
        arch.read_exact(&mut magic)?;
        assert_eq!("GENOMETP".as_bytes(), &magic);
        let version = read_u16(&mut arch)?;
        if !(218..219).contains(&version) {
            println!("Warning: untested Tple File version {version}");
        }

        let count = read_u32(&mut arch)? as usize;
        let mut entities = Vec::with_capacity(count);
        for _idx in 0..count {
            let entity = TemplateEntity::load(&mut arch)?;
            entities.push(entity);
        }

        let mut parents = Vec::new();
        loop {
            let parent = read_i32(&mut arch)?;
            let child = read_i32(&mut arch)?;
            parents.push((child, parent));
            if child == -1 && parent == -1 {
                break;
            }
        }

        println!(
            "finished at: {:x} / {:x}",
            arch.current_read_idx + 14,
            arch.data.len() + 14
        );

        Ok(Self { entities, parents })
    }

    pub fn save(&self, arch: &mut PakFile) -> Result<()> {
        let magic = "GENOMETP".as_bytes();
        arch.write_all(magic)?;
        let version = 219;
        write_u16(arch, version)?;
        write_u32(arch, self.entities.len() as u32)?;

        for entity in &self.entities {
            entity.save(arch)?;
        }
        assert_eq!(self.parents.last().unwrap(), &(-1, -1));
        for (child, parent) in &self.parents {
            write_i32(arch, *parent)?;
            write_i32(arch, *child)?;
        }
        Ok(())
    }
}
