use super::helpers::*;
use crate::error::*;
use std::io::{Read, Seek, Write};

pub struct PakFile {
    pub version: u16,
    pub data: Vec<u8>,
    pub strings: Vec<String>,
    pub current_read_idx: usize,
}

impl Default for PakFile {
    fn default() -> Self {
        Self::new()
    }
}

impl PakFile {
    pub fn new() -> Self {
        Self {
            version: 1,
            data: Vec::new(),
            strings: Vec::new(),
            current_read_idx: 0,
        }
    }

    pub fn load<I: ArchiveReadTarget>(src: &mut I) -> Result<Self> {
        let mut magic = [0u8; 8];
        src.read_exact(&mut magic)?;
        assert_eq!("GENOMFLE".as_bytes(), &magic);
        let version = read_u16(src)?;
        assert_eq!(version, 1);
        let offset = read_u32(src)?;
        let data_len = offset - (8 + 2 + 4); //data_len = offset - sizeof(header)
        let mut data = vec![0u8; data_len as usize];
        src.read_exact(&mut data)?;

        let text_magic = read_u32(src)?;
        assert_eq!(text_magic, 0xdeadbeef);
        let text_ver = read_u8(src)?;
        assert_eq!(text_ver, 1);
        let text_count = read_u32(src)? as usize;

        let mut strings = Vec::with_capacity(text_count);
        for _idx in 0..text_count {
            strings.push(src.read_str()?);
        }

        Ok(Self {
            version,
            data,
            strings,
            current_read_idx: 0,
        })
    }

    pub fn save<W: Write>(&self, dst: &mut W) -> Result<()> {
        let magic = "GENOMFLE".as_bytes();
        dst.write_all(magic)?;

        write_u16(dst, self.version)?;

        let data_len = self.data.len();
        let offset = data_len + (8 + 2 + 4); //data_len = offset - sizeof(header)
        write_u32(dst, offset as u32)?;

        dst.write_all(&self.data)?;

        let text_magic = 0xdeadbeef;
        write_u32(dst, text_magic)?;

        let text_ver = 1;
        write_u8(dst, text_ver)?;

        let text_count = self.strings.len();
        write_u32(dst, text_count as u32)?;

        for string in &self.strings {
            let (str_enc, _, unmappable_chars) = encoding_rs::WINDOWS_1252.encode(string);
            if unmappable_chars {
                println!("Warning: String '{string}' contains unmappable characters!")
            }
            write_u16(dst, str_enc.len() as u16)?;
            dst.write_all(&str_enc)?;
        }
        Ok(())
    }
}

impl Read for PakFile {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.write(&self.data[self.current_read_idx..])?;
        self.current_read_idx += len;
        Ok(len)
    }
}
impl Write for PakFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl Seek for PakFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        use std::io::*;
        self.current_read_idx = match pos {
            SeekFrom::Start(offset) => offset as usize,
            SeekFrom::End(offset) if -offset <= self.data.len() as i64 => {
                (self.data.len() as i64 + offset) as usize
            }
            SeekFrom::Current(offset) if -offset <= self.data.len() as i64 => {
                (self.current_read_idx as i64 + offset) as usize
            }
            _ => {
                return Err(ErrorKind::Unsupported.into());
            }
        };
        Ok(self.current_read_idx as u64)
    }
}

pub struct PakFileTempBlock<'a> {
    pub target: Box<&'a mut dyn ArchiveWriteTarget>,
    pub temp_data: Vec<u8>,
}

impl<'a> PakFileTempBlock<'a> {
    pub fn new<W: ArchiveWriteTarget>(target: &'a mut W) -> Self {
        Self {
            target: Box::new(target),
            temp_data: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.temp_data
    }
}

impl Write for PakFileTempBlock<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.temp_data.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub trait ArchiveReadTarget: Read + Seek {
    fn read_str(&mut self) -> Result<String> {
        // default impl for everything that supports Read

        let mut len_buf = [0u8; 2];
        self.read_exact(&mut len_buf)?;
        let len = u16::from_le_bytes(len_buf) as usize;
        let mut str_buf = vec![0; len];
        self.read_exact(&mut str_buf)?;
        if let Some(string) =
            encoding_rs::WINDOWS_1252.decode_without_bom_handling_and_without_replacement(&str_buf)
        {
            Ok(string.to_string())
        } else {
            Err(Error::InvalidString(format!("{:x?}", str_buf)))
        }
    }
}

impl ArchiveReadTarget for PakFile {
    fn read_str(&mut self) -> Result<String> {
        let id = read_u16(self)? as usize;
        let result = self.strings[id].as_str();

        Ok(result.to_string())
    }
}

impl ArchiveReadTarget for std::fs::File {}
impl ArchiveReadTarget for std::io::BufReader<std::fs::File> {}

pub trait ArchiveWriteTarget: Write {
    fn create_str_idx(&mut self, content: &str) -> Result<usize>;
    fn write_str(&mut self, content: &str) -> Result<()>;
}

impl ArchiveWriteTarget for PakFile {
    fn create_str_idx(&mut self, content: &str) -> Result<usize> {
        if let Some(idx) = self
            .strings
            .iter()
            .position(|existing| existing.as_str() == content)
        {
            Ok(idx)
        } else {
            let idx = self.strings.len();
            self.strings.push(content.to_string());
            Ok(idx)
        }
    }
    fn write_str(&mut self, content: &str) -> Result<()> {
        let idx = self.create_str_idx(content)?;
        write_u16(&mut self.data, idx as u16)?;
        Ok(())
    }
}

impl ArchiveWriteTarget for PakFileTempBlock<'_> {
    fn create_str_idx(&mut self, content: &str) -> Result<usize> {
        self.target.create_str_idx(content)
    }
    fn write_str(&mut self, content: &str) -> Result<()> {
        let idx = self.create_str_idx(content)?;
        write_u16(&mut self.temp_data, idx as u16)?;
        Ok(())
    }
}
