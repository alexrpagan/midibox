use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let out_dir = "./src/generated";
    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("midibox.bin"))
        .compile(&["./proto/midibox.proto"], &["."])?;
    Ok(())
}