use crate::audio::TrackSpec;
use wav::{Header, BitDepth};
use std::fs::File;
use std::u16;

// TODO: Accept data as Vec<f64> and convert to u16 or u8 here.
pub fn write_wav(data: Vec<u16>, track_spec: &TrackSpec, out_filename: &str) -> Result<(), String> {
    let header = Header::new(1, 1, track_spec.sample_rate() as u32, 16);
    let data = vec_to_i16(data);
    let out_file = File::create(out_filename);
    if let Err(e) = &out_file {
        return Err(format!("Unable to create output file {}: {:?}", out_filename, e));
    }
    let mut out_file = out_file.unwrap();
    let bd = BitDepth::Sixteen(data);
    let write_res = wav::write_wav(header, bd, &mut out_file);
    if let Err(e) = write_res {
        return Err(format!("Unable to write WAV file {}: {:?}", out_filename, e));
    };
    Ok(())
}

fn vec_to_i16(data: Vec<u16>) -> Vec<i16> {
    let mut res = Vec::with_capacity(data.len());
    let offset = ((u16::MAX as u32 + 1) / 2) as i32;
    for sample in data {
        res.push((sample as i32 - offset) as i16);
    }
    res
}