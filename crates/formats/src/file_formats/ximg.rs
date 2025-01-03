use image_dds::ddsfile::Dds;

use crate::{archive::ArchiveReadTarget, error::*, resourcefile::ResourceFile};

#[derive(Debug)]
pub struct XimgFile {
    pub res: ResourceFile,
    pub unknown1: [u8; 32],
    pub dds: Dds,
}

const R1_REV: [u8; 4] = *b"IM04";
const R1_CLASS: &str = "eCImageResource2";

impl XimgFile {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let res = ResourceFile::load(src)?;

        assert_eq!(&res.data_revision, &R1_REV);
        // there are multiple valid source exts
        assert_eq!(&res.class_name, &R1_CLASS);

        let mut unknown1 = [0; 32];
        src.read_exact(&mut unknown1)?;

        let dds = Dds::read(src).map_err(|e| Error::InvalidStructure(format!("{e}")))?;

        Ok(Self { res, dds, unknown1 })
    }

    // pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {

    //     let mut data = TempWriteTarget::new(dst);
    //     self.save_xmac(&mut data)?;
    //     let data = data.finish();

    //     Ok(())
    // }
}
