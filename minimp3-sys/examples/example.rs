extern crate minimp3_sys;

use std::{convert::TryInto, io};
use std::mem;
use std::ptr;

use std::io::Write;

fn decode_frame(
    ctx: &mut minimp3_sys::mp3dec_t,
    mp3_file: &[u8],
    pcm: &mut [i16],
    frame_info: &mut minimp3_sys::mp3dec_frame_info_t,
) -> Option<usize> {
    unsafe {
        let samples = minimp3_sys::mp3dec_decode_frame(
            ctx,
            mp3_file.as_ptr(),
            mp3_file.len() as _,
            pcm.as_mut_ptr(),
            frame_info,
        );

        match frame_info.frame_bytes {
            0 => None,
            _ => Some(samples as usize),
        }
    }
}

fn load_buf(
    ctx: &mut minimp3_sys::mp3dec_t,
    mp3_file: &[u8],
    file_info: &mut minimp3_sys::mp3dec_file_info_t,
) -> Result<(), i32> {
    let errno = unsafe {
        minimp3_sys::mp3dec_load_buf(
            ctx,
            mp3_file.as_ptr(),
            mp3_file.len() as _,
            file_info,
            None,
            ptr::null_mut(),
        )
    };

    match errno {
        0 => Ok(()),
        _ => Err(errno),
    }
}

fn main() {
    let mp3 = include_bytes!("../test.mp3");

    let mut context = unsafe { mem::zeroed() };

    unsafe { minimp3_sys::mp3dec_init(&mut context) };

    // output samples
    let mut pcm = vec![0i16; minimp3_sys::MINIMP3_MAX_SAMPLES_PER_FRAME as usize];

    // frame info
    let mut frame: minimp3_sys::mp3dec_frame_info_t = unsafe { mem::zeroed() };

    // file info
    let mut file: minimp3_sys::mp3dec_file_info_t = unsafe { mem::zeroed() };

    match load_buf(&mut context, &mp3[..], &mut file) {
        Ok(_) => println!("Success!"),
        Err(errno) => panic!("Errno => {}", errno),
    };

    let mut offset = 0usize;
    let mut stdout = io::stdout();

    let size = unsafe {
        let mut size = (mp3.len() - offset).try_into().unwrap();
        minimp3_sys::mp3dec_skip_id3(&mut mp3[offset..].as_ptr(), &mut size);
        size as _
    };

    while let Some(samples) = decode_frame(&mut context, &mp3[offset..size], &mut pcm, &mut frame) {
        //eprintln!("frame {:?}", frame);
        offset += frame.frame_bytes as usize;

        unsafe {
            use std::slice;

            let slice = slice::from_raw_parts(pcm.as_ptr() as _, samples * 2);
            stdout.write(slice).unwrap();
        }
    }
}
