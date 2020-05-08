use crate::audio::TrackSpec;
use crate::pitch::Note;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn parse(song_spec: &str) -> Result<(TrackSpec, Vec<ParsedFragment>), String> {
    let re = Regex::new(r"([a-gA-G][#b]?)(\d+)(([\+-]\d\d?\d?)?)").unwrap();
    let parser = Parser::new(re);
    parser.parse(song_spec)
}

struct Parser {
    pitch_spec_re: Regex,
}

impl Parser {
    fn new(pitch_spec_re: Regex) -> Parser {
        Parser { pitch_spec_re }
    }

    fn parse(&self, song_spec: &str) -> Result<(TrackSpec, Vec<ParsedFragment>), String> {
        let lines: Vec<_> = song_spec
            .lines()
            .map(|line| String::from(line.trim()))
            .filter(|line| !line.is_empty()) // empty lines
            .filter(|line| !line.starts_with("//")) // comments
            .collect();

        let (header_lines, data_lines) = extract_header_and_data(&lines)?;
        let track_spec = self.parse_header(&header_lines)?;
        let fragments = self.parse_data(&data_lines)?;
        Ok((track_spec, fragments))
    }

    fn parse_header(&self, header_lines: &[String]) -> Result<TrackSpec, String> {
        let mut values = HashMap::new();
        for line in header_lines {
            let split: Vec<_> = line.split('=').collect();
            if split.len() != 2 {
                return Err(format!(
                    "Line '{}' does not match format 'key=value'.",
                    line
                ));
            }
            values.insert(split[0], split[1]);
        }

        let sample_rate = parse_header_field::<u16>(&values, "sample_rate")?;
        let bpm = parse_header_field::<u16>(&values, "bpm")?;
        let subdivision = parse_header_field::<u8>(&values, "subdivision")?;
        let freq_a4 = parse_header_field::<f64>(&values, "freq_a4")?;
        let volume = parse_header_field::<f64>(&values, "volume")?;

        Ok(TrackSpec::new(
            sample_rate,
            bpm,
            subdivision,
            freq_a4,
            volume,
        ))
    }

    fn parse_data(&self, lines: &[String]) -> Result<Vec<ParsedFragment>, String> {
        let mut fragments = Vec::with_capacity(lines.len());
        for line in lines {
            fragments.push(self.parse_data_line(line)?)
        }
        Ok(fragments)
    }

    fn parse_data_line(&self, line: &str) -> Result<ParsedFragment, String> {
        let error_msg = format!("Invalid data line: {}", line);
        let tokens: Vec<_> = line.split(' ').filter(|token| !token.is_empty()).collect();
        if tokens.len() < 2 {
            return Err(format!(
                "Need at least two arguments per line - invalid line: {}",
                line
            ));
        }
        let note_val = convert_value::<u64>(&tokens[0], &error_msg)?;
        let mut pitches = Vec::with_capacity(tokens.len() - 1);
        for token in &tokens[1..] {
            pitches.push(self.parse_pitch(token)?)
        }
        Ok(ParsedFragment::new(note_val, pitches))
    }

    fn parse_pitch(&self, token: &str) -> Result<ParsedPitch, String> {
        if token == "-" {
            // This is a rest.
            // TODO: It would be better to encode this case in the regex as well.
            // TODO: This theoretically allows for chords with rests + non-rests.
            return Ok(ParsedPitch::rest());
        }

        let error_msg = format!("Invalid pitch spec: {}", token);
        let caps = match self.pitch_spec_re.captures(token) {
            Some(c) => c,
            None => return Err(error_msg),
        };

        // Find note representation.
        let note = match Note::for_name(caps.get(1).unwrap().as_str()) {
            Some(n) => n,
            None => return Err(error_msg),
        };

        // Find octave value.
        let octave = convert_value::<u8>(caps.get(2).unwrap().as_str(), &error_msg)?;

        //Find detune value
        let detune_spec = caps.get(3).unwrap().as_str();
        let detune = if detune_spec.is_empty() {
            0i8
        } else {
            convert_value::<i8>(detune_spec, "")?
        };

        Ok(ParsedPitch::new(note, octave, detune))
    }
}

fn extract_header_and_data(lines: &[String]) -> Result<(Vec<String>, Vec<String>), String> {
    let (idx_header, idx_data) = find_chunk_tag_indices(&lines)?;

    // TODO: It would be nice to MOVE lines, instead of copying them.
    let header_lines: Vec<_> = lines[idx_header + 1..idx_data]
        .iter()
        .map(String::from)
        .collect();

    // TODO: Same here.
    let data_lines: Vec<_> = lines[idx_data + 1..]
        .iter()
        .map(String::from)
        .collect();

    Ok((header_lines, data_lines))
}

fn find_chunk_tag_indices(lines: &[String]) -> Result<(usize, usize), String> {
    let mut idx_header = 0usize;
    let mut idx_data = 0usize;
    let mut header_tag_found = false;
    let mut data_tag_found = false;
    for (i, line) in lines.iter().enumerate() {
        if line == "#header" {
            if header_tag_found {
                // Duplicate #header tag.
                return Err("Need exactly one #header tag.".to_string());
            }
            idx_header = i;
            header_tag_found = true;
        }
        if line == "#data" {
            if data_tag_found {
                // Duplicate #data tag.
                return Err("Need exactly one #data tag.".to_string());
            }
            idx_data = i;
            data_tag_found = true;
        }
    }

    if !header_tag_found {
        return Err("Missing #header tag.".to_string());
    }
    if !data_tag_found {
        return Err("Missing #data tag.".to_string());
    }
    if idx_header >= idx_data {
        return Err("#header tag must come before #data tag.".to_string());
    }

    Ok((idx_header, idx_data))
}

fn parse_header_field<T: FromStr>(map: &HashMap<&str, &str>, key: &str) -> Result<T, String> {
    let value = map.get(key);
    let value = match value {
        Some(v) => *v,
        None => return Err(format!("Header field not found: {}.", key)),
    };
    let value = value.parse::<T>();
    let value = match value {
        Ok(v) => v,
        Err(_) => return Err(format!("Invalid type for header field: {}.", key)),
    };
    Ok(value)
}

fn convert_value<T: FromStr>(raw: &str, error_msg: &str) -> Result<T, String> {
    let value = raw.parse::<T>();
    match value {
        Ok(v) => Ok(v),
        Err(_) => Err(error_msg.to_string()),
    }
}

#[derive(Debug)]
pub struct ParsedFragment {
    value: u64,
    pitches: Vec<ParsedPitch>,
}

impl ParsedFragment {
    fn new(value: u64, pitches: Vec<ParsedPitch>) -> ParsedFragment {
        ParsedFragment { value, pitches }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn pitches(&self) -> &[ParsedPitch] {
        &self.pitches
    }
}

#[derive(Debug)]
pub struct ParsedPitch {
    note: Note,
    octave: u8,
    detune: i8,
}

impl ParsedPitch {
    fn new(note: Note, octave: u8, detune: i8) -> ParsedPitch {
        ParsedPitch {
            note,
            octave,
            detune,
        }
    }

    pub fn rest() -> ParsedPitch {
        ParsedPitch {
            note: Note::Rest,
            octave: 1,
            detune: 0,
        }
    }

    pub fn note(&self) -> Note {
        self.note
    }

    pub fn octave(&self) -> u8 {
        self.octave
    }

    pub fn detune(&self) -> i8 {
        self.detune
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use std::fs;

    const VALID_FILES_PATH: &str = "input_files/";
    const INVALID_FILES_PATH: &str = "input_files/invalid/";

    fn read_file_or_panic(filename: &str) -> String {
        fs::read_to_string(filename).expect(&format!("File not found: {}", { filename }))
    }

    fn valid_file(filename: &str) -> String {
        read_file_or_panic(&format!("{}{}", VALID_FILES_PATH, filename))
    }

    fn invalid_file(filename: &str) -> String {
        read_file_or_panic(&format!("{}{}", INVALID_FILES_PATH, filename))
    }

    #[test]
    fn missing_header_tag() {
        let contents = invalid_file("missing_header_tag.wss");
        let res = parse::parse(&contents);
        assert!(res.is_err(), "Expect result to be error.");
    }

    #[test]
    fn missing_data_tag() {
        let contents = invalid_file("missing_data_tag.wss");
        let res = parse::parse(&contents);
        assert!(res.is_err(), "Expect result to be error.");
    }

    #[test]
    fn missing_both_tags() {
        let contents = invalid_file("missing_both_tags.wss");
        let res = parse::parse(&contents);
        assert!(res.is_err(), "Expect result to be error.");
    }
}
