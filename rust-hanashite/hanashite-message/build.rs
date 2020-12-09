extern crate prost_build;

use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(::std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src").join("protos");

    let in_dir = PathBuf::from(::std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("protos");
    println!("{:?}", &in_dir);
    // Re-run this build.rs if the protos dir changes (i.e. a new file is added)
    println!("cargo:rerun-if-changed={}", in_dir.to_str().unwrap());

    // Find all *.proto files in the `in_dir` and add them to the list of files
    let mut protos = Vec::new();
    let proto_ext = Some(Path::new("proto").as_os_str());
    for entry in std::fs::read_dir(&in_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == proto_ext {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            protos.push(path);
        }
    }
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir(out_dir);
    println!("{:?}", protos);
    prost_build.compile_protos(&protos, &[in_dir]).unwrap();
}
