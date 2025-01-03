use std::{
    collections::VecDeque,
    env,
    ffi::OsString,
    fs::File,
    io::{BufWriter, Seek, Write},
    path::Path,
};

use formats::file_formats::xmac::XmacFile;
use serde::Serialize;
mod translation;

fn main() {
    println!("Chromosome Toolkit - R1 - XMAC to GLTF");
    let mut queue = env::args().skip(1).collect::<VecDeque<_>>();
    let mut dump_intermediate = false;
    let mut include_textures = false;
    while let Some(arg) = queue.pop_front() {
        if &arg == "/dumpintermediate" {
            dump_intermediate = true;
            continue;
        }
        if &arg == "/includetextures" {
            include_textures = true;
            continue;
        }
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
                    if meta.is_dir() || path.ends_with("._xmac") {
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
        let xmac = XmacFile::load(&mut in_data).unwrap();
        println!(
            "Read: {:x}/{:x}",
            in_data.stream_position().unwrap(),
            in_data.seek(std::io::SeekFrom::End(0)).unwrap()
        );
        drop(in_data);

        let out_arg = arg.replace("._xmac", ".gltf");
        if out_arg == arg {
            panic!("In == out path");
        }
        if dump_intermediate {
            println!("Dumping intermediate format");
            let int_arg = arg.replace("._xmac", "._xmac.json");
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
            let int_os = OsString::from(&int_arg);
            let int_file = File::create(Path::new(&int_os)).expect("Unable to open output file");
            let mut int_file = BufWriter::new(int_file);
            let mut ser = serde_json::Serializer::with_formatter(&mut int_file, formatter);
            xmac.serialize(&mut ser).unwrap();
            int_file.flush().unwrap();
        }
        let out_bin = OsString::from(arg.replace("._xmac", ".bin"));

        let gltf = translation::xmac_to_gltf(&xmac, Path::new(&out_bin), include_textures).unwrap();
        println!("Translation done");

        let out_os = OsString::from(&out_arg);
        let out_path = Path::new(&out_os);

        let out_file = File::create(out_path).expect("Unable to open output file");
        let mut out_file = BufWriter::new(out_file);

        gltf.to_writer_pretty(&mut out_file).unwrap();
        out_file.flush().unwrap();

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
