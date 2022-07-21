use std::{env, path::PathBuf};

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("filer_descriptor.bin"))
        .compile(&["filer_pb.proto"], &["src/protos"])
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

