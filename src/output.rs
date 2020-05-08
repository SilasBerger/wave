use crate::audio::TrackSpec;
use std::fs::File;
use std::i16;
use wav::{BitDepth, Header};

pub fn write_wav(data: Vec<f64>, track_spec: &TrackSpec, out_filename: &str) -> Result<(), String> {
    let header = Header::new(1, 1, track_spec.sample_rate() as u32, 16);
    let data = vec_to_i16(data);
    let out_file = File::create(out_filename);
    if let Err(e) = &out_file {
        return Err(format!(
            "Unable to create output file {}: {:?}",
            out_filename, e
        ));
    }
    let mut out_file = out_file.unwrap();
    let bd = BitDepth::Sixteen(data);
    let write_res = wav::write_wav(header, bd, &mut out_file);
    if let Err(e) = write_res {
        return Err(format!(
            "Unable to write WAV file {}: {:?}",
            out_filename, e
        ));
    };
    Ok(())
}

fn vec_to_i16(data: Vec<f64>) -> Vec<i16> {
    data.into_iter()
        .map(|sample| ((sample * i16::MAX as f64) + i16::MIN as f64).round() as i16)
        .collect()
}
