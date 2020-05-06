use crate::audio::TrackSpec;
use std::collections::HashMap;
use std::str::FromStr;

pub fn parse(song_spec: &str) -> Result<(), String> {
    let lines: Vec<_> = song_spec.lines()
        .into_iter()
        .map(|line| String::from(line.trim()))
        .filter(|line| !line.is_empty()) // empty lines
        .filter(|line| !line.starts_with("//")) // comments
        .collect();

    let (header_lines, data_lines) = extract_header_and_data(&lines)?;
    let track_spec = parse_header(&header_lines);

    Ok(())
}

fn extract_header_and_data(lines: &Vec<String>) -> Result<(Vec<String>, Vec<String>), String> {
    let (idx_header, idx_data) = find_chunk_tag_indices(&lines)?;

    // TODO: It would be nice to MOVE lines, instead of copying them.
    let header_lines: Vec<_> = lines[idx_header+1..idx_data].iter()
        .map(|line| String::from(line))
        .collect();

    // TODO: Same here.
    let data_lines: Vec<_> = lines[idx_data+1.. ].iter()
        .map(|line| String::from(line))
        .collect();

    Ok((header_lines, data_lines))
}

fn find_chunk_tag_indices(lines: &Vec<String>) -> Result<(usize, usize), String> {
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
        return Err("#header tag must come before #data tag.".to_string())
    }

    Ok((idx_header, idx_data))
}

fn parse_header(header_lines: &Vec<String>) -> Result<TrackSpec, String> {
    let mut values = HashMap::new();
    for line in header_lines {
        let split: Vec<_> = line.split("=").collect();
        if split.len() != 2 {
            return Err(format!("Line '{}' does not match format 'key=value'.", line));
        }
        values.insert(split[0], split[1]);
    }

    let sample_rate = blub::<u16>(&values)?;
    println!("Sample rate: {}", sample_rate);

    Err("".to_string())
}

fn blub<T: FromStr>(map: &HashMap<&str, &str>) -> Result<T, String> {
    let value = map.get("sample_rate");
    let value = match value {
        Some(v) => *v,
        None => return Err("Missing key.".to_string())
    };
    let sample_rate = value.parse::<T>();
    let sample_rate = match sample_rate {
        Ok(v) => v,
        Err(_) => return Err("sdf".to_string())
    };
    Ok(sample_rate)
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::parse;

    const VALID_FILES_PATH: &str = "input_files/";
    const INVALID_FILES_PATH: &str = "input_files/invalid/";

    fn read_file_or_panic(filename: &str) -> String {
        fs::read_to_string(filename).expect(&format!("File not found: {}", {filename}))
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