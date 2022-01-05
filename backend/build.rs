fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Compiling grpc stuff for backend");
    tonic_build::compile_protos("../rpc-proto/crumb.proto")?;
    Ok(())
}
