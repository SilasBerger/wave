use wave::util;
use std::{fs, process};

fn main() {
    from_input_file("input_files/the_licc_with_rest.wss");
}

#[allow(dead_code)]
fn from_input_file(filename: &str) {
    let contents = read_input_or_exit(filename);
    let wav_encoded = match wave::text_to_wav_bytes(&contents) {
        Ok(we) => we,
        Err(e) => {
            eprintln!("Failed to get wav encoded data for file {}: {}", filename, e);
            return;
        }
    };
    write_to_file("output.wav", &wav_encoded);
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