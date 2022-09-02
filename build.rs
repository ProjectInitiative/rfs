use std::fs;
use std::{env, path::PathBuf};

fn main() {
    // pull output directory from env variable
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // find all .proto files
    let paths = fs::read_dir("src/protos").unwrap();
    // convert to vec of Strings
    let proto_files = paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("protobuf_descriptor.bin"))
        .compile(proto_files.as_slice(), &["src/protos"])
        .unwrap();

    // tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    // // Use this in build.rs
    // protobuf_codegen::Codegen::new()
    //     // use pure rust parser, optional.
    //     // .pure()
    //     // Use `protoc` parser, optional.
    //     // .protoc()
    //     // Use `protoc-bin-vendored` bundled protoc command, optional.
    //     .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
    //     // All inputs and imports from the inputs must reside in `includes` directories.
    //     .includes(&["src/protos"])
    //     // Inputs must reside in some of include paths.
    //     .input("src/protos/filer_pb.proto")
    //     // Specify output directory relative to Cargo output directory.
    //     .out_dir("src/protos")
    //     // .cargo_out_dir("protos")
    //     .run_from_script();
}
