fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["./protos"])
        // Inputs must reside in some of include paths.
        .input("./protos/hello.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("src/protos")
        //.cargo_out_dir("protos")
        .run_from_script();
}
