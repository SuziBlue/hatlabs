fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:warning=OUT_DIR is {}", out_dir); // <--- this prints during build

    prost_build::compile_protos(
        &["onnx/onnx.proto"],
        &["onnx"],
    )?;

    Ok(())
}
