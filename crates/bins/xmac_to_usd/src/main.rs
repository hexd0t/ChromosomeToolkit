use std::{
    collections::VecDeque,
    env,
    ffi::OsString,
    io::{Seek, Write},
    path::Path,
};

use formats::xmac::XmacFile;
use serde::Serialize;

fn main() {
    println!("Chromosome Toolkit - R1 - XMAC to USD");
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

        let out_arg = arg.replace("._xmac", "._xmac.json");
        if out_arg == arg {
            panic!("In == out path");
        }
        let out_os = OsString::from(&out_arg);
        let out_path = std::path::Path::new(&out_os);

        let out_file = std::fs::File::create(out_path).expect("Unable to open output file");
        let mut out_file = std::io::BufWriter::new(out_file);

        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(&mut out_file, formatter);
        xmac.serialize(&mut ser).unwrap();
        out_file.flush().unwrap();
        println!("done");
    }
}
