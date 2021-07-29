fn main() {
    tonic_build::configure()
        .build_client(true)
        .compile(&["../app/proto/app.proto"],
                 &["../app/proto/"])
        .unwrap();
}
