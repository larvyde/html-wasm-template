use base64::write::EncoderWriter;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufWriter, Write};

const INPUT_FILE: &str = "pkg/placeholder_name_replace_me_bg.wasm";
const OUTPUT_FILE: &str = "pkg/placeholder_name_replace_me_bg.wasm.b64.js";

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut in_file = BufReader::new(File::open(INPUT_FILE)?);
    let mut out_file = BufWriter::new(File::create(OUTPUT_FILE)?);
    write!(out_file, "wasm_bindgen.module = \"data:application/wasm;base64,")?;
    {
        let mut out = EncoderWriter::new(&mut out_file, &base64::engine::general_purpose::STANDARD);
        std::io::copy(&mut in_file, &mut out)?;
    }
    writeln!(out_file, "\";")?;
    Ok(())
}