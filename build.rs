use std::{env, fs};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);


    let out_dir = "./src/generated";
    fs::create_dir_all(out_dir)?;
    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("midibox.bin"))
        .compile(&["./proto/midibox.proto"], &["."])?;
    Ok(())
}