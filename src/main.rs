use minimp3::{Decoder, Frame, Error};

use std::fs::File;

fn main() {
    let argv = std::env::args();
    let path = argv.skip(1).next().unwrap();
    let mut decoder = Decoder::new(File::open(&path).unwrap());

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                println!("Decoded {} samples", data.len() / channels)
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
}

