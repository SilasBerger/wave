use wave::audio;
use wave::audio::{FragmentSpec, TrackSpec};
use wave::util;
use wave::pitch::{PitchGenerator, TwelveTET, Note};
use wave::output;

const SAMPLE_RATE: u16 = 44100;

fn main() {
    the_licc();
}

#[allow(dead_code)]
fn with_rest() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 60, 8, 440.0);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let g = pitch_gen.get(Note::G, 4);
    let a = pitch_gen.get(Note::A, 4);
    let c = pitch_gen.get(Note::C, 5);
    let d = pitch_gen.get(Note::D, 5);
    let e = pitch_gen.get(Note::E, 5);
    let fragments = vec![
        FragmentSpec::note(1, a, 0.5),
        FragmentSpec::note(1, a, 0.5),
        FragmentSpec::note(1, c, 0.5),
        FragmentSpec::note(1, c, 0.5),
        FragmentSpec::note(1, e, 0.5),
        FragmentSpec::note(1, e, 0.5),
        FragmentSpec::note(2, d, 0.5),
        FragmentSpec::rest(2),

        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, d, 0.5),
        FragmentSpec::note(1, d, 0.5),
        FragmentSpec::note(2, c, 0.5),
        FragmentSpec::rest(2),
    ];
    bounce_and_export("silence.wav", &fragments, &track_spec);
}

#[allow(dead_code)]
fn the_licc() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 180, 8, 440.0);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let g = pitch_gen.get(Note::G, 4);
    let a = pitch_gen.get(Note::A, 4);
    let b = pitch_gen.get(Note::B, 4);
    let c = pitch_gen.get(Note::C, 5);
    let d = pitch_gen.get(Note::D, 5);

    let fragments = vec![
        FragmentSpec::note(1, a, 0.5),
        FragmentSpec::note(1, b, 0.5),
        FragmentSpec::note(1, c, 0.5),
        FragmentSpec::note(1, d, 0.5),
        FragmentSpec::note(2, b, 0.5),
        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, a, 0.5)
    ];
    bounce_and_export("the_licc.wav", &fragments, &track_spec);
}

#[allow(dead_code)]
fn a_major_with_melody() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 80, 4, 440.0);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let a = pitch_gen.get(Note::A, 4);
    let c_sharp = pitch_gen.get(Note::CSharp, 5);
    let e = pitch_gen.get(Note::E, 5);
    let fragments = vec![
        FragmentSpec::note(1, a, 0.5),
        FragmentSpec::note(1, c_sharp, 0.5),
        FragmentSpec::note(1, e, 0.3),
        FragmentSpec::chord(2, vec![a, c_sharp, e], 0.5)
    ];
    bounce_and_export("melody_and_chord.wav", &fragments, &track_spec);
}

fn bounce_and_export(filename: &str, fragments: &[FragmentSpec], track_spec: &TrackSpec) {
    let data = audio::bounce(fragments, track_spec);
    let wav_encoded = output::encode_wav(data, &track_spec);
    write_to_file(filename, &wav_encoded);
}

fn write_to_file(filename: &str, buf: &[u8]) {
    if let Err(e) = util::write_to_file(filename, buf) {
        eprintln!("Error writing file {}: {:?}", filename, e);
    } else {
        println!("Successfully wrote file {}", filename);
    }
}

