// Format: http://www.bendlins.de/nico/risen/risensnd.txt

use super::helpers::*;
use bitflags::bitflags;
use std::{
    io::{Cursor, Write},
    time::UNIX_EPOCH,
};

pub struct SndFile {
    props: SndFileProps,
    mp3_data: Vec<u8>,
}

bitflags! {
    pub struct SoundFlags: u8 {
        const Is2D = 0b0000001;
        const IsSoftware = 0b00000010;
    }
}

impl SndFile {
    pub fn new(props: SndFileProps, mp3_data: Vec<u8>) -> Self {
        Self { props, mp3_data }
    }

    pub fn write(&self) -> Vec<u8> {
        let props = self.props.write();
        let mut w = Cursor::new(Vec::with_capacity(self.mp3_data.len() + 0xBF)); //0xBF is typical header size

        //res_magic: [u8; 4], // "GR01"
        w.write_all("GR01".as_bytes()).unwrap();
        //res_class: [u8; 4], // "SN04"
        w.write_all("SN04".as_bytes()).unwrap();
        //prop_offset: u32 - Header up to that point is always 0x28
        write_u32(&mut w, 0x28).unwrap();
        //prop_length: u32,
        write_u32(&mut w, props.len() as u32).unwrap();
        //data_offset: u32,
        write_u32(&mut w, 0x28 + props.len() as u32).unwrap();
        //data_length: u32,
        write_u32(&mut w, self.mp3_data.len() as u32).unwrap();
        //raw_timestamp: u64,    // (FILETIME)
        write_u64(&mut w, Self::filetime()).unwrap();
        //raw_file_ext: [u8; 8], // ".wav"*/
        w.write_all(".wav".as_bytes()).unwrap();
        w.write_all(&[0, 0, 0, 0]).unwrap();

        w.write_all(&props).unwrap();
        w.write_all(&self.mp3_data).unwrap();
        w.into_inner()
    }

    fn filetime() -> u64 {
        use std::time::SystemTime;
        const FILE_TIMES_PER_SEC: u64 = 10_000_000;
        const UNIX_EPOCH_OFFSET: u64 = 134_774 * 86400;
        let now_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_windows_s = now_unix + UNIX_EPOCH_OFFSET;
        now_windows_s * FILE_TIMES_PER_SEC
    }
}

pub struct SndFileProps {
    duration: u32,
    flags: SoundFlags,
    //Links not implemented
}

impl SndFileProps {
    pub fn new(duration: u32, flags: SoundFlags) -> Self {
        Self { duration, flags }
    }

    pub fn write(&self) -> Vec<u8> {
        let contents = vec![
            Self::write_prop("Duration", "long", &self.duration.to_le_bytes()),
            Self::write_prop("Flags", "char", &[self.flags.bits()]),
            Self::write_prop(
                "Links",
                "bTObjArray<struct eCSoundResource2::SLink>",
                &[1, 0, 0, 0, 0],
            ), // Version 1, 0 links
        ];

        let mut w = Cursor::new(Vec::with_capacity(0x97));
        // Header:
        w.write_all(&[1, 0, 1, 1, 0, 1]).unwrap();
        // ClassName:
        const CLASS_NAME: &str = "eCSoundResource2";
        write_u16(&mut w, CLASS_NAME.len() as u16).unwrap();
        w.write_all(CLASS_NAME.as_bytes()).unwrap();
        // unknown:
        w.write_all(&[1, 0, 0]).unwrap();
        // class version:
        write_u16(&mut w, 0xC9).unwrap();
        // version:
        write_u16(&mut w, 0xC9).unwrap();
        // data size:
        let data_size: usize = 2 + 4 + contents.iter().map(|d| d.len()).sum::<usize>() + 2;
        write_u32(&mut w, data_size as u32).unwrap();
        // data version:
        write_u16(&mut w, 0xC9).unwrap();
        // property count:
        write_u32(&mut w, contents.len() as u32).unwrap();
        for content in contents {
            w.write_all(&content).unwrap();
        }
        // class version:
        write_u16(&mut w, 0xC9).unwrap();

        w.into_inner()
    }

    fn write_prop(name: &str, p_type: &str, data: &[u8]) -> Vec<u8> {
        let size = 2 + 2 + 2 + 4 + name.len() + p_type.len() + data.len();
        let mut w = Cursor::new(Vec::with_capacity(size));
        write_u16(&mut w, name.len() as u16).unwrap();
        w.write_all(name.as_bytes()).unwrap();
        write_u16(&mut w, p_type.len() as u16).unwrap();
        w.write_all(p_type.as_bytes()).unwrap();
        write_u16(&mut w, 0x1E).unwrap();
        write_u32(&mut w, data.len() as u32).unwrap();
        w.write_all(data).unwrap();
        w.into_inner()
    }
}
