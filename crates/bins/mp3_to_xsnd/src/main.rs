use formats::xsnd::{SndFile, SndFileProps, SoundFlags};
use std::{collections::VecDeque, env, ffi::OsString, path::Path};
use symphonia::core::{
    audio::AudioBufferRef,
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};

fn main() {
    println!("Chromosome Toolkit - R1 - MP3 to XSND");
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
                    if meta.is_dir() || path.ends_with(".mp3") {
                        queue.push_back(path);
                    }
                }
            } else {
                println!("Reading dir failed");
            }
            continue;
        }

        let dur = get_dur(path);
        println!("Duration: {dur}");
        let mp3_data = std::fs::read(path).unwrap();
        let mut flags = SoundFlags::empty();
        if arg.contains("_sw") {
            flags.insert(SoundFlags::IsSoftware);
        }
        if arg.contains("_2d") {
            flags.insert(SoundFlags::Is2D);
            flags.insert(SoundFlags::IsSoftware);
        }

        let sndfile = SndFile::new(SndFileProps::new(dur as u32, flags), mp3_data);
        let out_arg = arg.replace(".mp3", "._xsnd");
        if out_arg == arg {
            panic!("In == out path");
        }
        let out_os = OsString::from(&out_arg);
        let out_path = std::path::Path::new(&out_os);

        if out_path.exists() {
            println!("exists");
            continue;
        }
        std::fs::write(out_path, sndfile.write()).unwrap();
        println!("done");
    }
}

fn get_dur(path: &Path) -> u64 {
    let src = std::fs::File::open(path).expect("failed to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    let sample_rate = track.codec_params.sample_rate.expect("need sample rate");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    let mut sample_count = 0;
    // The decode loop.
    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(error))
                if error.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => panic!("{}", e),
        };
        if packet.buf().is_empty() {
            break;
        }
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }
        if packet.track_id() != track_id {
            continue;
        }
        let data = decoder.decode(&packet).unwrap();
        match data {
            AudioBufferRef::F32(buf) => {
                sample_count += buf.planes().planes()[0].len();
            }
            _ => unimplemented!(),
        }
    }

    sample_count as u64 * 1000 / sample_rate as u64
}
