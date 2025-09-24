fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("resources/stepper_service.proto")?;
    Ok(())
}