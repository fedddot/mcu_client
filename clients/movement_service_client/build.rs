fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("resources/movement_vendor_api.proto")?;
    Ok(())
}