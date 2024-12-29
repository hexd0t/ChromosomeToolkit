#![allow(dead_code)]
use std::io::{Read, Result, Write};

pub fn write_bool<W: Write>(w: &mut W, val: bool) -> Result<()> {
    let val = if val { 1 } else { 0 };
    write_u8(w, val)
}
pub fn write_u8<W: Write>(w: &mut W, val: u8) -> Result<()> {
    w.write_all(&[val; 1])
}
pub fn write_u16<W: Write>(w: &mut W, val: u16) -> Result<()> {
    w.write_all(&val.to_le_bytes())
}
pub fn write_i16<W: Write>(w: &mut W, val: i16) -> Result<()> {
    w.write_all(&val.to_le_bytes())
}
pub fn write_u32<W: Write>(w: &mut W, val: u32) -> Result<()> {
    w.write_all(&val.to_le_bytes())
}
pub fn write_i32<W: Write>(w: &mut W, val: i32) -> Result<()> {
    w.write_all(&val.to_le_bytes())
}
pub fn write_u64<W: Write>(w: &mut W, val: u64) -> Result<()> {
    w.write_all(&val.to_le_bytes())
}
pub fn write_f32<W: Write>(w: &mut W, val: f32) -> Result<()> {
    w.write_all(&val.to_le_bytes())
}

pub fn read_bool<R: Read>(r: &mut R) -> Result<bool> {
    Ok(read_u8(r)? != 0)
}
pub fn read_u8<R: Read>(r: &mut R) -> Result<u8> {
    let mut data = [0u8; 1];
    r.read_exact(&mut data)?;
    Ok(data[0])
}
pub fn read_u16<R: Read>(r: &mut R) -> Result<u16> {
    let mut data = [0u8; 2];
    r.read_exact(&mut data)?;
    Ok(u16::from_le_bytes(data))
}
pub fn read_i16<R: Read>(r: &mut R) -> Result<i16> {
    let mut data = [0u8; 2];
    r.read_exact(&mut data)?;
    Ok(i16::from_le_bytes(data))
}
pub fn read_u32<R: Read>(r: &mut R) -> Result<u32> {
    let mut data = [0u8; 4];
    r.read_exact(&mut data)?;
    Ok(u32::from_le_bytes(data))
}
pub fn read_i32<R: Read>(r: &mut R) -> Result<i32> {
    let mut data = [0u8; 4];
    r.read_exact(&mut data)?;
    Ok(i32::from_le_bytes(data))
}
pub fn read_u64<R: Read>(r: &mut R) -> Result<u64> {
    let mut data = [0u8; 8];
    r.read_exact(&mut data)?;
    Ok(u64::from_le_bytes(data))
}
pub fn read_f32<R: Read>(r: &mut R) -> Result<f32> {
    let mut data = [0u8; 4];
    r.read_exact(&mut data)?;
    Ok(f32::from_le_bytes(data))
}

pub fn read_u16_endian<R: Read>(r: &mut R, big_endian: bool) -> Result<u16> {
    let mut data = [0u8; 2];
    r.read_exact(&mut data)?;
    if big_endian {
        Ok(u16::from_be_bytes(data))
    } else {
        Ok(u16::from_le_bytes(data))
    }
}
pub fn read_u32_endian<R: Read>(r: &mut R, big_endian: bool) -> Result<u32> {
    let mut data = [0u8; 4];
    r.read_exact(&mut data)?;
    if big_endian {
        Ok(u32::from_be_bytes(data))
    } else {
        Ok(u32::from_le_bytes(data))
    }
}
pub fn read_i32_endian<R: Read>(r: &mut R, big_endian: bool) -> Result<i32> {
    let mut data = [0u8; 4];
    r.read_exact(&mut data)?;
    if big_endian {
        Ok(i32::from_be_bytes(data))
    } else {
        Ok(i32::from_le_bytes(data))
    }
}
pub fn read_f32_endian<R: Read>(r: &mut R, big_endian: bool) -> Result<f32> {
    let mut data = [0u8; 4];
    r.read_exact(&mut data)?;
    if big_endian {
        Ok(f32::from_be_bytes(data))
    } else {
        Ok(f32::from_le_bytes(data))
    }
}
pub fn write_f32_endian<W: Write>(w: &mut W, val: f32, big_endian: bool) -> Result<()> {
    if big_endian {
        w.write_all(&val.to_be_bytes())
    } else {
        w.write_all(&val.to_le_bytes())
    }
}

pub mod ser_hex {
    use std::fmt::Write;

    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let mut hex_str = String::with_capacity(v.len() * 2);
        for e in v {
            hex_str.write_str(format!("{e:02x}").as_str()).unwrap();
        }
        String::serialize(&hex_str, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let hex_str = String::deserialize(d)?;
        let mut result = Vec::with_capacity(hex_str.len() / 2);
        for idx in 0..result.capacity() {
            let char_idx = idx * 2;
            if let Some(entry) = hex_str.get(char_idx..char_idx + 2) {
                match u8::from_str_radix(entry, 16) {
                    Ok(val) => result.push(val),
                    Err(err) => panic!("Error parsing hex digits '{entry}' to byte: {err}"),
                }
            } else {
                panic!("Hex string has uneven digits!");
            }
        }
        Ok(result)
    }
}
