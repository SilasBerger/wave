use wave::sine;
use wave::sine::{FragmentSpec, TrackSpec};
use wave::util;
use wave::pitch::{pitch, Note};
use wave::output;

const SAMPLE_RATE: u16 = 44100;

fn main() {
    a_major_with_melody();
}

#[allow(dead_code)]
fn the_licc() {
    let g = pitch(Note::G, 4);
    let a = pitch(Note::A, 4);
    let b = pitch(Note::B, 4);
    let c = pitch(Note::C, 5);
    let d = pitch(Note::D, 5);
    let track_spec = TrackSpec::new(SAMPLE_RATE, 180, 8);
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
    let a = pitch(Note::A, 4);
    let c_sharp = pitch(Note::CSharp, 5);
    let e = pitch(Note::E, 5);
    let track_spec = TrackSpec::new(SAMPLE_RATE, 80, 4);
    let fragments = vec![
        FragmentSpec::note(1, a, 0.5),
        FragmentSpec::note(1, c_sharp, 0.5),
        FragmentSpec::note(1, e, 0.3),
        FragmentSpec::chord(2, vec![a, c_sharp, e], 0.5)
    ];
    bounce_and_export("melody_and_chord.wav", &fragments, &track_spec);
}

fn bounce_and_export(filename: &str, fragments: &[FragmentSpec], track_spec: &TrackSpec) {
    let data = sine::bounce(fragments, track_spec);
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

