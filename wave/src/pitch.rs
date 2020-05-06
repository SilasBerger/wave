const A4: f64 = 440.0;

pub enum Note {
    A,
    ASharp,
    BFlat,
    B,
    C,
    CSharp,
    DFlat,
    D,
    DSharp,
    EFlat,
    E,
    F,
    FSharp,
    GFlat,
    G,
    GSharp,
    AFlat
}

pub trait PitchGenerator {
    fn get(&self, note: Note, octave: u8) -> f64;
    fn det(&self, note: Note, octave: u8, det_cents: i8) -> f64;
}

pub struct TwelveTET {
    freq_a4: f64
}

impl TwelveTET {
    pub fn new(freq_a4: f64) -> TwelveTET {
        TwelveTET {
            freq_a4
        }
    }

    fn pitch_index(&self, note: Note, octave: u8) -> i32 {
        let base_index = match note {
            Note::C => 1,
            Note::CSharp => 2,
            Note::DFlat => 2,
            Note::D => 3,
            Note::DSharp => 4,
            Note::EFlat => 4,
            Note::E => 5,
            Note::F => 6,
            Note::FSharp => 7,
            Note::GFlat => 7,
            Note::G => 8,
            Note::GSharp => 9,
            Note::AFlat => 9,
            Note::A => 10,
            Note::ASharp => 11,
            Note::BFlat => 11,
            Note::B => 12
        };
        base_index + (octave * 12) as i32
    }
}

impl PitchGenerator for TwelveTET {
    fn get(&self, note: Note, octave: u8) -> f64 {
        let delta_index = self.pitch_index(note, octave) - self.pitch_index(Note::A, 4);
        self.freq_a4 * 2.0f64.powf(1.0 / 12.0).powi(delta_index)
    }

    fn det(&self, note: Note, octave: u8, det_cents: i8) -> f64 {
        let index_det = self.pitch_index(note, octave) as f64 + (det_cents as f64 / 100.0);
        let delta_index = index_det - self.pitch_index(Note::A, 4) as f64;
        self.freq_a4 * 2.0f64.powf(1.0 / 12.0).powf(delta_index)
    }
}
