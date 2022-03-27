fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/")
        .inputs(&["src/geosite.proto"])
        .include(".")
        .run()
        .expect("protoc");
}
