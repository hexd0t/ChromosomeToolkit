use formats::archive::PakFile;
use formats::tple::TpleFile;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{env, ffi::OsString, path::Path};

fn main() {
    println!("Chromosome Toolkit - R1 - JSON to TPLE");
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
                    if meta.is_dir() || path.ends_with(".tple.json") {
                        queue.push_back(path);
                    }
                }
            } else {
                println!("Reading dir failed");
            }
            continue;
        }

        let in_data = File::open(path).unwrap();
        let in_data = BufReader::new(in_data);

        let tple: TpleFile = match serde_json::from_reader(in_data) {
            Ok(r) => r,
            Err(e) => {
                println!("Parsing JSON failed: {e}, skipping");
                continue;
            }
        };
        println!("Parsing finished");

        // As long as some contents remain unparsed, we need to retain the original string indices:
        let string_src = arg.replace(".tple.json", ".tple");
        let mut in_data = File::open(string_src).unwrap();
        let PakFile {
            version: _,
            data: _,
            strings: prefill_strings,
            current_read_idx: _,
        } = match PakFile::load(&mut in_data) {
            Ok(a) => a,
            Err(e) => {
                println!("loading string src archive failed: {e}, skipping");
                continue;
            }
        };
        drop(in_data);
        println!("Loading orig strings finished");

        let mut arch = PakFile::new();
        arch.strings = prefill_strings;
        match tple.save(&mut arch) {
            Ok(_) => {}
            Err(e) => {
                println!("writing tple failed: {e}, skipping");
                continue;
            }
        };

        println!(
            "Archive: {} bytes, {} strings",
            arch.data.len(),
            arch.strings.len()
        );

        let out_arg = arg.replace(".tple.json", "_out.tple");
        if out_arg == arg {
            panic!("In == out path");
        }
        let out_os = OsString::from(&out_arg);
        let out_path = Path::new(&out_os);

        // if out_path.exists() {
        //     println!("exists");
        //     continue;
        // }
        let mut out_file = File::create(out_path).expect("Unable to open output file");
        match arch.save(&mut out_file) {
            Ok(_) => {}
            Err(e) => {
                println!("writing archive failed: {e}");
            }
        }
        out_file.flush().unwrap();
        println!("done");
    }
}
