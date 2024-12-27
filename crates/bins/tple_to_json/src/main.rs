use formats::archive::PakFile;
use formats::tple::TpleFile;
use serde::Serialize;
use std::collections::VecDeque;
use std::io::Write;
use std::{env, ffi::OsString, path::Path};

fn main() {
    println!("Chromosome Toolkit - R1 - TPLE to JSON");
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
                    if meta.is_dir() || path.ends_with(".tple") || path.ends_with(".TPLE") {
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
        let arch = match PakFile::load(&mut in_data) {
            Ok(a) => a,
            Err(e) => {
                println!("loading archive failed: {e}, skipping");
                continue;
            }
        };
        drop(in_data);

        println!(
            "Archive: {} bytes, {} strings",
            arch.data.len(),
            arch.strings.len()
        );

        let tple = match TpleFile::load(arch) {
            Ok(a) => a,
            Err(e) => {
                println!("loading tple failed: {e}, skipping");
                continue;
            }
        };
        println!("Parsing finished");
        //println!("{:?}", lrent.root);

        let out_arg = arg.replace(".tple", ".tple.json");
        if out_arg == arg {
            panic!("In == out path");
        }
        let out_os = OsString::from(&out_arg);
        let out_path = std::path::Path::new(&out_os);

        // if out_path.exists() {
        //     println!("exists");
        //     continue;
        // }
        let mut out_file = std::fs::File::create(out_path).expect("Unable to open output file");

        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(&mut out_file, formatter);
        tple.serialize(&mut ser).unwrap();
        out_file.flush().unwrap();
        println!("done");
    }
}
