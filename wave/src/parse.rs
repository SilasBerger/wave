pub fn parse(song_spec: &str) -> Result<(), String> {
    let lines: Vec<_> = song_spec.lines()
        .into_iter()
        .map(|line| String::from(line.trim()))
        .filter(|line| !line.is_empty()) // empty lines
        .filter(|line| !line.starts_with("//")) // comments
        .collect();

    let (header_lines, data_lines) = extract_header_and_data(&lines)?;

    println!("\nHeader lines:");
    for line in header_lines {
        println!("{}", line)
    }

    println!("\nData lines:");
    for line in data_lines {
        println!("{}", line);
    }

    Ok(())
}

fn extract_header_and_data(lines: &Vec<String>) -> Result<(Vec<String>, Vec<String>), String> {
    let (idx_header, idx_data) = find_chunk_tag_indices(&lines)?;

    // TODO: It would be nice if this could just take ownership of the strings it needs, rather
    // than create copies. The two collections are disjoint anyway. Is there a way for us to move
    // a string out of a vec and still have access to the "rest" of that vec?
    let header_lines: Vec<String> = lines.iter()
        .skip_while(|line| *line != "#header")
        .take_while(|line| *line != "#data")
        .map(|line| String::from(line))
        .collect();

    // TODO: Same here - copying would be better than moving
    let data_lines: Vec<_> = lines.iter()
        .skip_while(|line| *line != "#data")
        .skip(1)
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
    if idx_data >= idx_header {
        return Err("#header tag must come before #data tag.".to_string())
    }

    Ok((idx_header, idx_data))
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