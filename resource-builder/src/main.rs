mod serializers;

use std::{path::Path, fs};

use entities::{symphony_job::Symphony};
use kube::CustomResourceExt;
use serializers::serialize_to_yaml_file;

fn main() -> anyhow::Result<()> {
    let output_folder = Path::new("resources");
    fs::create_dir_all(&output_folder)?;

    let crd_output_path = output_folder.join("symphony-crd.yaml");
    let crd = Symphony::crd();

    serialize_to_yaml_file(&crd, &crd_output_path)?;

    Ok(())
}
