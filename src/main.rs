use wave::util;
use std::{fs, process};
use std::env;

fn main() {
    let cmd_args = read_cmd_args();
    if let None = &cmd_args {
        eprintln!("Invalid number of arguments: Specify at least an input filename.");
        return;
    }
    let (in_filename, out_filename) = cmd_args.unwrap();
    process_and_export(&in_filename, &out_filename);
}

fn read_cmd_args() -> Option<(String, String)> {
    let args: Vec<_> = env::args().collect();
    let in_filename = args.get(1)?;
    let default_out_filename = String::from("output.wav");
    let out_filename = args.get(2).unwrap_or(&default_out_filename);
    Some((String::from(in_filename), String::from(out_filename)))
}

fn process_and_export(in_filename: &str, out_filename: &str) {
    let contents = read_input_or_exit(in_filename);
    if let Err(e) = wave::text_to_wav_export(&contents, out_filename) {
        eprintln!("Error: {}", e);
    } else {
        println!("Successfully exported {}", out_filename);
    }
}

#[allow(dead_code)]
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