use minimp3::{Decoder, Error, Frame};

use std::fs::File;

fn main() {
    let mut decoder =
        Decoder::new(File::open("minimp3-sys/test.mp3").unwrap());

    loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => println!("Decoded {} samples", data.len() / channels),
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
}
