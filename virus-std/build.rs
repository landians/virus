extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config
        .out_dir("src/protocol")
        .compile_protos(&["src/protocol/protocol.proto"], &["src/"])
        .unwrap();
}
