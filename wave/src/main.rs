use wave::sine;
use wave::sine::{FragmentSpec, TrackSpec};
use wave::util;
use wave::pitch::{pitch, Note};

const SAMPLE_RATE: u16 = 44100;

fn main() {
    the_licc();
}

#[allow(dead_code)]
fn the_licc() {
    let g = pitch(Note::G, 4);
    let a = pitch(Note::A, 4);
    let b = pitch(Note::B, 4);
    let c = pitch(Note::C, 5);
    let d = pitch(Note::D, 5);
    let track_spec = TrackSpec::new(SAMPLE_RATE, 60, 8);
    let fragments = vec![
        FragmentSpec::note(1, a, 0.5),
        FragmentSpec::note(1, b, 0.5),
        FragmentSpec::note(1, c, 0.5),
        FragmentSpec::note(1, d, 0.5),
        FragmentSpec::note(2, b, 0.5),
        FragmentSpec::note(1, g, 0.5),
        FragmentSpec::note(1, a, 0.5)
    ];
    let wave = sine::bounce(&fragments, &track_spec);
    write_to_file("the_licc.pcm", &wave);
}

#[allow(dead_code)]
fn a_major_with_melody() {
    let a = pitch(Note::A, 4);
    let c_sharp = pitch(Note::CSharp, 5);
    let e = pitch(Note::E, 5);
    let fragments = vec![
        FragmentSpec::note(500, a, 0.5),
        FragmentSpec::note(500, c_sharp, 0.5),
        FragmentSpec::note(500, e, 0.3),
        FragmentSpec::chord(1000, vec![a, c_sharp, e], 0.5)
    ];
    let track_spec = TrackSpec::new(SAMPLE_RATE, 80, 8);
    let wave = sine::bounce(&fragments, &track_spec);
    write_to_file("melody_and_chord.pcm", &wave);
}

#[allow(dead_code)]
fn a_major() {
    let fragment = FragmentSpec::chord(1000,
                                       vec![440.0, 554.37, 659.25],
                                       0.8);
    let track_spec = TrackSpec::new(SAMPLE_RATE, 80, 8);
    let wave = sine::bounce(&vec![fragment], &track_spec);
    write_to_file("new_power_chord.pcm", &wave);
}

fn write_to_file(filename: &str, buf: &[u8]) {
    if let Err(e) = util::write_to_file(filename, buf) {
        eprintln!("Error writing file {}: {:?}", filename, e);
    } else {
        println!("Successfully wrote file {}", filename);
    }
}

