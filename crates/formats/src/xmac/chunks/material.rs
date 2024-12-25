use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use super::XmacChunkMeta;

use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;
use crate::types::Vector4;
use crate::xmac::read_xmac_str;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacStdMaterial {
    pub name: String,
    pub layers: Vec<XmacStandardMaterialLayer>,

    pub ambient_color: Vector4,
    pub diffuse_color: Vector4,
    pub specular_color: Vector4,
    pub emissive_color: Vector4,
    pub shine: f32,
    pub shine_strength: f32,
    pub opacity: f32,
    pub refraction_index: f32,
    pub double_sided: bool,
    pub wireframe: bool,
    pub transparency_type: XmacMaterialTransparencyType,
}

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacMaterialTransparencyType {
    Filter = b'F',
    Subtractive = b'S',
    Additive = b'A',
    Unknown = b'U',
}

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacLayerBlendMode {
    /// The foreground texture covers up the background texture entirely.
    None = 0,
    /// The foreground texture is applied like a decal to the background.
    /// The shape of the decal is determined by the foreground alpha.
    Over = 1,
    /// The result is the background texture cut in the shape of the foreground alpha.
    In = 2,
    /// The result is the opposite of In.
    /// It is as if the shape of the foreground alpha has been cut out of the background.
    Out = 3,
    /// The result color is the foreground color added to the background color as if being projected on the background through a slide projector.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Add = 4,
    /// The result color is the foreground color subtracted from the background color.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Subtract = 5,
    /// The result color is the foreground color multiplied by the background color.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Multiply = 6,
    /// The result color is the difference between the foreground color and the background color.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Difference = 7,
    /// The result color of each pixel is the background color or foreground color, whichever is lighter.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Lighten = 8,
    /// The result color of each pixel is the background color or foreground color, whichever is darker.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Darken = 9,
    /// The result color is the background color with saturation increased in proportion to the foreground color scaled by foreground alpha.
    /// If the foreground color is red, for example, the result color will be the background color with more saturated reds.
    Saturate = 10,
    /// The result color is the background color with saturation decreased in proportion to the foreground color scaled by foreground alpha.
    /// If the foreground color is red, for example, the result color will be the background color with desaturated reds.
    Desaturate = 11,
    /// The result color is the background color mixed with the foreground color, brighter where the foreground is bright and darker where the foreground is dark.
    /// It is as if the foreground texture represents the light falling on the background.
    /// The result color is then applied over the background color using the foreground alpha to define the opacity of the result.
    Illuminate = 12,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacStandardMaterialLayer {
    #[serde(rename = "type")]
    pub ty: XmacMaterialLayerType,
    pub texture: String,

    pub amount: f32,
    pub u_offset: f32,
    pub v_offset: f32,
    pub u_tiling: f32,
    pub v_tiling: f32,
    pub rotation_rads: f32,
    pub material_id: u16,
    pub blend_mode: XmacLayerBlendMode,
}

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacMaterialLayerType {
    Unknown = 0,
    Ambient = 1,
    Diffuse = 2,
    Specular = 3,
    Opacity = 4,
    /// Contains a normal-map
    Bump = 5,
    SelfIllumination = 6,
    /// shininess (for specular)
    Shine = 7,
    /// shine strength (for specular)
    ShineStrength = 8,
    FilterColor = 9,
    Reflect = 10,
    Refract = 11,
    Environment = 12,
    Displacement = 13,
}

impl XmacStdMaterial {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading STD MATERIAL chunk...");
        match chunk_meta.version {
            2 => {
                let ambient_color = Vector4::load_endian(src, big_endian)?;
                let diffuse_color = Vector4::load_endian(src, big_endian)?;
                let specular_color = Vector4::load_endian(src, big_endian)?;
                let emissive_color = Vector4::load_endian(src, big_endian)?;
                let shine = read_f32_endian(src, big_endian)?;
                let shine_strength = read_f32_endian(src, big_endian)?;
                let opacity = read_f32_endian(src, big_endian)?;
                let refraction_index = read_f32_endian(src, big_endian)?;
                let double_sided = read_bool(src)?;
                let wireframe = read_bool(src)?;
                let transparency_type = XmacMaterialTransparencyType::try_from(read_u8(src)?)?;
                let layer_count = read_u8(src)?;
                let material_name = read_xmac_str(src, big_endian)?;
                let mut layers = Vec::with_capacity(layer_count as usize);
                for _idx in 0..layer_count {
                    let amount = read_f32_endian(src, big_endian)?;
                    let u_offset = read_f32_endian(src, big_endian)?;
                    let v_offset = read_f32_endian(src, big_endian)?;
                    let u_tiling = read_f32_endian(src, big_endian)?;
                    let v_tiling = read_f32_endian(src, big_endian)?;
                    let rotation_rads = read_f32_endian(src, big_endian)?;
                    let material_id = read_u16_endian(src, big_endian)?;

                    let layer_type = XmacMaterialLayerType::try_from(read_u8(src)?)?;
                    let blend_mode = XmacLayerBlendMode::try_from(read_u8(src)?)?;

                    let texture = read_xmac_str(src, big_endian)?;

                    layers.push(XmacStandardMaterialLayer {
                        texture,
                        ty: layer_type,
                        amount,
                        u_offset,
                        v_offset,
                        u_tiling,
                        v_tiling,
                        rotation_rads,
                        blend_mode,
                        material_id,
                    });
                }
                Ok(Some(Self {
                    name: material_name,
                    layers,

                    ambient_color,
                    diffuse_color,
                    specular_color,
                    emissive_color,
                    shine,
                    shine_strength,
                    opacity,
                    refraction_index,
                    double_sided,
                    wireframe,
                    transparency_type,
                }))
            }
            ver => {
                println!(
                    "Unknown XMAC materials version {ver}@{:x}, skipping",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}
