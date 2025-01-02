use std::{
    collections::VecDeque,
    env,
    ffi::OsString,
    fs::File,
    io::{BufWriter, Seek, Write},
    path::Path,
};

use formats::ximg::XimgFile;
use serde::Serialize;

fn main() {
    println!("Chromosome Toolkit - R1 - XIMG to PNG");
    let mut queue = env::args().skip(1).collect::<VecDeque<_>>();
    while let Some(arg) = queue.pop_front() {
        println!("{}", arg);
        let os_arg = OsString::from(&arg);
        let path = Path::new(&os_arg);
        if !path.exists() {
            println!("not found");
            continue;
        }

        if path.is_dir() {
            if let Ok(dir) = path.read_dir() {
                for file in dir.flatten() {
                    let meta = file.metadata().unwrap();
                    let path = file.path().to_string_lossy().to_string();
                    if meta.is_dir() || path.ends_with("._ximg") {
                        queue.push_back(path);
                    }
                }
            } else {
                println!("Reading dir failed");
            }
            continue;
        }
        let in_data = std::fs::File::open(path).unwrap();
        let mut in_data = std::io::BufReader::new(in_data);
        let ximg = XimgFile::load(&mut in_data).unwrap();
        println!(
            "Read: {:x}/{:x}",
            in_data.stream_position().unwrap(),
            in_data.seek(std::io::SeekFrom::End(0)).unwrap()
        );
        drop(in_data);

        let out_arg = arg.replace("._ximg", ".png");
        if out_arg == arg {
            panic!("In == out path");
        }

        let image = image_dds::image_from_dds(&ximg.dds, 0).unwrap();
        println!("Conversion finished");
        image
            .save_with_format(out_arg, image_dds::image::ImageFormat::Png)
            .unwrap();

        println!("done");
    }
}

#[derive(Debug)]
pub enum ConvError {
    NotImplemented(String),
    MandatoryDataMissing(String),
    InvalidData(String),
    IoError(std::io::Error),
}

type Result<T> = std::result::Result<T, ConvError>;

impl std::error::Error for ConvError {}
impl std::fmt::Display for ConvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<std::io::Error> for ConvError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
