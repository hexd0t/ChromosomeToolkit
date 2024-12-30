use std::{
    collections::VecDeque,
    env,
    ffi::OsString,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    time::SystemTime,
};

mod translation;

fn main() {
    println!("Chromosome Toolkit - R1 - GLTF to XMAC");
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
                    if meta.is_dir() || path.ends_with(".gltf") || path.ends_with(".glb") {
                        queue.push_back(path);
                    }
                }
            } else {
                println!("Reading dir failed");
            }
            continue;
        }
        let file_time = path
            .metadata()
            .ok()
            .and_then(|meta| meta.modified().ok())
            .unwrap_or_else(SystemTime::now);
        let (gltf, buffer, textures) = gltf::import(path).unwrap();

        let out_arg = arg
            .replace(".gltf", "_out._xmac")
            .replace(".glb", "_out._xmac");
        if out_arg == arg {
            panic!("In == out path");
        }

        let xmac = translation::gltf_to_xmac(gltf, buffer, textures, file_time).unwrap();
        println!("Translation done");

        let out_os = OsString::from(&out_arg);
        let out_path = Path::new(&out_os);

        let out_file = File::create(out_path).expect("Unable to open output file");
        let mut out_file = BufWriter::new(out_file);

        xmac.save(&mut out_file).unwrap();
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
