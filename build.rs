use prost_build;

fn main() {
    prost_build::compile_protos(&["src/mixxx/proto/beats.proto"], &["src/mixxx/proto/"]).unwrap();
}
