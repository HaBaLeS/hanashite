extern crate prost_build;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let out_dir = Path::new("src/protos").to_path_buf();

    println!("OUTPUT: {}", out_dir.to_str().expect("fail"));
    let in_dir = PathBuf::from(::std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("protos");
    // Re-run this build.rs if the protos dir changes (i.e. a new file is added)
    println!("cargo:rerun-if-changed={}", in_dir.to_str().unwrap());

    // Find all *.proto files in the `in_dir` and add them to the list of files
    let mut protos = Vec::new();
    let proto_ext = Some(Path::new("proto").as_os_str());
    for entry in WalkDir::new(&in_dir) {
        let path = entry.unwrap().into_path();
        if path.extension() == proto_ext {
            // Re-run this build.rs if any of the files in the protos dir change
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            protos.push(path);
        }
    }
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir(out_dir);
    prost_build.type_attribute(".", "#[derive(Eq)]");
    prost_build.compile_protos(&protos,    &[in_dir]).unwrap();
}