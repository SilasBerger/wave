#[derive(Debug, Copy, Clone)]
pub enum Note {
    A,
    ASharp,
    BFlat,
    B,
    BSharp,
    CFlat,
    C,
    CSharp,
    DFlat,
    D,
    DSharp,
    EFlat,
    E,
    ESharp,
    FFlat,
    F,
    FSharp,
    GFlat,
    G,
    GSharp,
    AFlat,
    Rest,
}

impl Note {
    pub fn for_name(name: &str) -> Option<Note> {
        match name {
            "A" => Some(Note::A),
            "A#" => Some(Note::ASharp),
            "Bb" => Some(Note::BFlat),
            "B" => Some(Note::B),
            "B#" => Some(Note::BSharp),
            "Cb" => Some(Note::CFlat),
            "C" => Some(Note::C),
            "C#" => Some(Note::CSharp),
            "Db" => Some(Note::DFlat),
            "D" => Some(Note::D),
            "D#" => Some(Note::DSharp),
            "Eb" => Some(Note::EFlat),
            "E" => Some(Note::E),
            "E#" => Some(Note::ESharp),
            "Fb" => Some(Note::FFlat),
            "F" => Some(Note::F),
            "F#" => Some(Note::FSharp),
            "Gb" => Some(Note::GFlat),
            "G" => Some(Note::G),
            "G#" => Some(Note::GSharp),
            "Ab" => Some(Note::AFlat),
            _ => None,
        }
    }
}

pub trait PitchGenerator {
    fn get(&self, note: Note, octave: u8) -> f64;
    fn det(&self, note: Note, octave: u8, det_cents: i8) -> f64;
}

pub struct TwelveTET {
    freq_a4: f64,
}

impl TwelveTET {
    pub fn new(freq_a4: f64) -> TwelveTET {
        TwelveTET { freq_a4 }
    }

    fn pitch_index(&self, note: Note, octave: u8) -> i32 {
        let base_index = match note {
            Note::Rest => 0,
            Note::BSharp => 1,
            Note::C => 1,
            Note::CSharp => 2,
            Note::DFlat => 2,
            Note::D => 3,
            Note::DSharp => 4,
            Note::EFlat => 4,
            Note::E => 5,
            Note::ESharp => 6,
            Note::FFlat => 5,
            Note::F => 6,
            Note::FSharp => 7,
            Note::GFlat => 7,
            Note::G => 8,
            Note::GSharp => 9,
            Note::AFlat => 9,
            Note::A => 10,
            Note::ASharp => 11,
            Note::BFlat => 11,
            Note::B => 12,
            Note::CFlat => 12,
        };
        base_index + (octave * 12) as i32
    }
}

impl PitchGenerator for TwelveTET {
    fn get(&self, note: Note, octave: u8) -> f64 {
        if let Note::Rest = &note {
            return 0.0;
        }
        let delta_index = self.pitch_index(note, octave) - self.pitch_index(Note::A, 4);
        self.freq_a4 * 2.0f64.powf(1.0 / 12.0).powi(delta_index)
    }

    fn det(&self, note: Note, octave: u8, det_cents: i8) -> f64 {
        if let Note::Rest = &note {
            return 0.0;
        }
        let index_det = self.pitch_index(note, octave) as f64 + (det_cents as f64 / 100.0);
        let delta_index = index_det - self.pitch_index(Note::A, 4) as f64;
        self.freq_a4 * 2.0f64.powf(1.0 / 12.0).powf(delta_index)
    }
}
