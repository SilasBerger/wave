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

fn pitch_index(note: Note, register: u8) -> i32 {
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
    base_index + (register * 12) as i32
}

pub fn pitch(note: Note, register: u8) -> f64 {
    let delta_index = pitch_index(note, register) - pitch_index(Note::A, 4);
    A4 * (2.0 as f64).powf(1.0 / 12.0).powi(delta_index)
}
