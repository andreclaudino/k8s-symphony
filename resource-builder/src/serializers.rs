use std::{fs::File, io::Write, path::Path};

use serde::Serialize;


pub fn serialize_to_yaml_file<C: Serialize>(element: &C, file_path: &Path) -> anyhow::Result<()> {
    log::info!("Generating YAMl file for {file_path:?}");
    let generated_yaml = serde_yaml::to_string(&element)?;
    let mut output_file = File::create(&file_path)?;
    output_file.write(&generated_yaml.as_bytes())?;

    return Ok(())
}