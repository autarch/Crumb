fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/grpc")
        .compile(&["../rpc-proto/crumb.proto"], &["../rpc-proto"])?;
    Ok(())
}
