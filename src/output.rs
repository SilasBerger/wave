// With lots of help from https://www.youtube.com/watch?v=odeWLp96fdo.

use crate::audio::TrackSpec;

pub fn encode_wav(mut audio: Vec<u8>, track: &TrackSpec) -> Vec<u8> {
    let mut buf = Vec::new();
    add_riff_header(&mut buf, &audio);
    add_wave_header(&mut buf, &audio, track);
    buf.append(&mut audio);
    buf
}

fn add_riff_header(buf: &mut Vec<u8>, audio: &Vec<u8>) {
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&make_u32(20 + audio.len() as u32)); // WAVE chunk size
}

fn add_wave_header(buf: &mut Vec<u8>, audio: &Vec<u8>, track: &TrackSpec) {
    // WAVE chunk
    buf.extend_from_slice(b"WAVE");

    // fmt chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&make_u32(16)); // fmt chunk size
    buf.extend_from_slice(&make_u16(1)); // format code (PCM)
    buf.extend_from_slice(&make_u16(1)); // number of channels
    buf.extend_from_slice(&make_u32(track.sample_rate() as u32)); // sample rate
    buf.extend_from_slice(&make_u32(track.sample_rate() as u32)); // data rate
    buf.extend_from_slice(&make_u16(1)); // block size
    buf.extend_from_slice(&make_u16(8)); // bits per sample

    // data chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&make_u32(audio.len() as u32)); // data chunk size
}

// TODO: Use crate for this.
fn make_u32(v: u32) -> [u8; 4] {
    // TODO: This is not straightforward - document.
    let mut b = [0u8; 4];
    for i in 0..4 {
        b[i] = ((v >> (8 * i) as u32) & 0xff) as u8;
    }
    b
}

// TODO: Use crate for this.
fn make_u16(v: u16) -> [u8; 2] {
    // TODO: This is not straightforward - document.
    let mut b = [0u8; 2];
    for i in 0..2 {
        b[i] = ((v >> (8 * i) as u16) & 0xff) as u8;
    }
    b
}