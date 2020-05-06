use wave::{audio, parse};
use wave::audio::{FragmentSpec, TrackSpec};
use wave::util;
use wave::pitch::{PitchGenerator, TwelveTET, Note};
use wave::output;
use wave::parse::{ParsedFragment, ParsedPitch};
use std::{fs, process};

const SAMPLE_RATE: u16 = 44100;

fn main() {
    basic_input_file();
}

#[allow(dead_code)]
fn basic_input_file() {
    let filename = "input_files/the_licc_harmonized.wss";
    let contents = read_input_or_exit(filename);
    process_input_file(filename, &contents);
}

fn process_input_file(filename: &str, contents: &str) {
    // TODO: Capture errors.
    // TODO: Accept output filename and / or take from input filename.
    let (track_spec, fragments) = match parse::parse(&contents) {
        Ok(tup) => tup,
        Err(e) => {
            eprintln!("Failed to parse input file {}: {}", filename, e);
            return;
        }
    };
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let frag_specs = assemble_fragments(&fragments, &track_spec, &pitch_gen);
    bounce_and_export("output.wav", &frag_specs, &track_spec);
}

fn assemble_fragments(parsed_fragments: &Vec<ParsedFragment>, track_spec: &TrackSpec, pitch_gen: &dyn PitchGenerator) -> Vec<FragmentSpec> {
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

#[allow(dead_code)]
fn sound_of_silence() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 60, 8, 440.0, 0.5);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let g = pitch_gen.get(Note::G, 4);
    let a = pitch_gen.get(Note::A, 4);
    let c = pitch_gen.get(Note::C, 5);
    let d = pitch_gen.get(Note::D, 5);
    let e = pitch_gen.get(Note::E, 5);
    let fragments = vec![
        FragmentSpec::note(1, a, track_spec.volume()),
        FragmentSpec::note(1, a, track_spec.volume()),
        FragmentSpec::note(1, c, track_spec.volume()),
        FragmentSpec::note(1, c, track_spec.volume()),
        FragmentSpec::note(1, e, track_spec.volume()),
        FragmentSpec::note(1, e, track_spec.volume()),
        FragmentSpec::note(2, d, track_spec.volume()),
        FragmentSpec::rest(2),

        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, d, track_spec.volume()),
        FragmentSpec::note(1, d, track_spec.volume()),
        FragmentSpec::note(2, c, track_spec.volume()),
    ];
    bounce_and_export("silence.wav", &fragments, &track_spec);
}

#[allow(dead_code)]
fn the_licc_det() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 180, 8, 440.0, 0.5);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let g = pitch_gen.get(Note::G, 4);
    let a = pitch_gen.det(Note::A, 4, 0);
    let b = pitch_gen.det(Note::B, 4, 30);
    let b_2 = pitch_gen.det(Note::B, 4, -30);
    let c = pitch_gen.det(Note::C, 5, -12);
    let d = pitch_gen.det(Note::D, 5, -20);

    let fragments = vec![
        FragmentSpec::note(1, a, track_spec.volume()),
        FragmentSpec::note(1, b, track_spec.volume()),
        FragmentSpec::note(1, c, track_spec.volume()),
        FragmentSpec::note(1, d, track_spec.volume()),
        FragmentSpec::note(2, b_2, track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, a, track_spec.volume())
    ];
    bounce_and_export("the_licc_det.wav", &fragments, &track_spec);
}

#[allow(dead_code)]
fn the_licc_with_chords() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 180, 8, 440.0, 0.5);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let g = pitch_gen.get(Note::G, 4);
    let a = pitch_gen.get(Note::A, 4);
    let b = pitch_gen.get(Note::B, 4);
    let c = pitch_gen.get(Note::C, 5);
    let c_low = pitch_gen.get(Note::C, 4);
    let d = pitch_gen.get(Note::D, 5);
    let e = pitch_gen.get(Note::E, 5);
    let e_low = pitch_gen.get(Note::E, 4);
    let f_low = pitch_gen.get(Note::F, 4);

    let fragments = vec![
        FragmentSpec::chord(1, vec![a, c_low, e_low], track_spec.volume()),
        FragmentSpec::note(1, b, track_spec.volume()),
        FragmentSpec::note(1, c, track_spec.volume()),
        FragmentSpec::note(1, d, track_spec.volume()),
        FragmentSpec::chord(2, vec![b, d, f_low], track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::chord(2, vec![a, c, e, g], track_spec.volume())
    ];
    bounce_and_export("the_licc_with_chords.wav", &fragments, &track_spec);
}

#[allow(dead_code)]
fn the_licc() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 180, 8, 440.0, 0.5);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let g = pitch_gen.get(Note::G, 4);
    let a = pitch_gen.get(Note::A, 4);
    let b = pitch_gen.get(Note::B, 4);
    let c = pitch_gen.get(Note::C, 5);
    let d = pitch_gen.get(Note::D, 5);

    let fragments = vec![
        FragmentSpec::note(1, a, track_spec.volume()),
        FragmentSpec::note(1, b, track_spec.volume()),
        FragmentSpec::note(1, c, track_spec.volume()),
        FragmentSpec::note(1, d, track_spec.volume()),
        FragmentSpec::note(2, b, track_spec.volume()),
        FragmentSpec::note(1, g, track_spec.volume()),
        FragmentSpec::note(1, a, track_spec.volume())
    ];
    bounce_and_export("the_licc.wav", &fragments, &track_spec);
}

#[allow(dead_code)]
fn a_major_with_melody() {
    let track_spec = TrackSpec::new(SAMPLE_RATE, 80, 4, 440.0, 0.5);
    let pitch_gen = TwelveTET::new(track_spec.freq_a4());
    let a = pitch_gen.get(Note::A, 4);
    let c_sharp = pitch_gen.get(Note::CSharp, 5);
    let e = pitch_gen.get(Note::E, 5);
    let fragments = vec![
        FragmentSpec::note(1, a, track_spec.volume()),
        FragmentSpec::note(1, c_sharp, track_spec.volume()),
        FragmentSpec::note(1, e, track_spec.volume() - 0.2),
        FragmentSpec::chord(2, vec![a, c_sharp, e], track_spec.volume())
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

fn read_input_or_exit(filename: &str) -> String {
    let contents = fs::read_to_string(filename);
    if let Err(e) = contents {
        eprintln!("Unable to read input file {}: {}", filename, e);
        process::exit(1);
    }
    contents.unwrap()
}

