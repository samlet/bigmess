extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &[],
        input: &["helloworld.proto", "addressbook.proto", "simple.proto",
            "common_types.proto", "nlpserv.proto", "mess.proto", "metainfo.proto",
            "values.proto", "services_common.proto"
        ],
        rust_protobuf: true,
        ..Default::default()
    }).expect("protoc-rust-grpc");
}

