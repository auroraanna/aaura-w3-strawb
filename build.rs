use std::{
    env,
    path::Path,
    fs::write
};

fn main() {
    let maud_version: String = cargo_metadata::MetadataCommand::new().exec().unwrap().packages.iter().find_map(|package| {
        if package.name == "maud" {
            Some(package.version.to_string())
        } else {
            None
        }
    }).unwrap();

    write(
        Path::new(&env::var("OUT_DIR").unwrap())
            .join("maud_version.txt"),
        maud_version
    ).unwrap();
}
