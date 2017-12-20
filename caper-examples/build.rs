extern crate protoc_rust_caper;

fn main() {
    protoc_rust_caper::run(protoc_rust_caper::Args {
        out_dir: "src/protos",
        input: &[
            "src/protos/echo.proto",
            "src/protos/http_hello.proto",
            "src/protos/benchmark.proto",
            "src/protos/demo.proto",
        ],
        includes: &[],
        rust_protobuf: true,
    }).expect("Compile proto files in echo example failed.");
}
