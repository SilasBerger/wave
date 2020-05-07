use crate::pitch::{TwelveTET, PitchGenerator};
use crate::parse::ParsedFragment;
use crate::audio::{TrackSpec, FragmentSpec};
use std::u16;

pub mod pitch;
pub mod audio;
pub mod util;
pub mod output;
pub mod parse;

pub fn text_to_raw_audio(text: &str) -> Result<Vec<u16>, String> {
    let (track_spec, fragments) = parse::parse(&text)?;
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let fragments = assemble_fragments(&fragments, &track_spec, &pitch_gen);
    build_raw_audio(&fragments, &track_spec)
}

pub fn export_wav_from_text(text: &str, out_filename: &str) -> Result<(), String> {
    let (track_spec, fragments) = parse::parse(&text)?;
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let fragments = assemble_fragments(&fragments, &track_spec, &pitch_gen);
    let data = build_raw_audio(&fragments, &track_spec)?;
    output::write_wav(data, &track_spec, out_filename)
}



fn build_raw_audio(fragments: &Vec<FragmentSpec>, track_spec: &TrackSpec) -> Result<Vec<u16>, String> {
    Ok(audio::bounce(&fragments, &track_spec))
}

fn assemble_fragments(parsed_fragments: &Vec<ParsedFragment>,
                      track_spec: &TrackSpec,
                      pitch_gen: &dyn PitchGenerator) -> Vec<FragmentSpec> {
    let mut fragment_specs = Vec::with_capacity(parsed_fragments.len());
    for parsed_fragment in parsed_fragments {
        let mut pitches = Vec::with_capacity(parsed_fragment.pitches().len());
        for parsed_pitch in parsed_fragment.pitches() {
            let freq = pitch_gen.det(parsed_pitch.note(), parsed_pitch.octave(), parsed_pitch.detune());
            pitches.push(freq);
        }
        fragment_specs.push(FragmentSpec::chord(parsed_fragment.value(), pitches, track_spec.volume()));
    }
    return fragment_specs
}