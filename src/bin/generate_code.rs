use std::{env::set_var, path::PathBuf};

fn main() {
    let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("mixxx")
        .join("schema");

    // Set the OUT_DIR environment variable, which is otherwise set by cargo when executing
    // the `build.rs`.
    set_var("OUT_DIR", target_dir.to_string_lossy().to_string());
    prost_build::compile_protos(&["src/mixxx/proto/beats.proto"], &["src/mixxx/proto/"]).unwrap();
}
