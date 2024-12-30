use std::time::SystemTime;

use formats::xmac::XmacFile;

use super::Result;

pub fn gltf_to_xmac(
    gltf: gltf::Document,
    buffer: Vec<gltf::buffer::Data>,
    textures: Vec<gltf::image::Data>,
    file_time: SystemTime,
) -> Result<XmacFile> {
    let mut result = XmacFile::new(file_time);
    // translate_nodes(input, &mut outputs)?;
    // translate_materials(input, &mut outputs)?;
    // translate_meshes(input, &mut outputs)?;
    // translate_skinning(input, &mut outputs)?;
    // translate_morphs(input, &mut outputs)?;

    //result.set_bounding

    Ok(result)
}
