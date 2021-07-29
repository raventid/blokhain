fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(&["../app/proto/app.proto"],
                 &["../app/proto/"])
        .unwrap();
}
