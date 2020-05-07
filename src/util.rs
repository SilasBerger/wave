use std::fs::File;
use std::io::{Write, Error};

pub fn write_to_file(filename: &str, buf: &[u8]) -> Result<(), Error>{
    let mut output_file = File::create(filename)?;
    output_file.write_all(buf)?;
    Ok(())
}