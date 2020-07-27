use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_env = env::var("CARGO_CFG_TARGET_ENV")?;
    println!("Current Target env={}", current_env);
    if current_env.to_lowercase() == "musl" {
        // we don't need to build for musl
        return Ok(());
    }
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/wa.proto");
    prost_build::Config::new()
        .out_dir("src/")
        .compile_protos(&["src/wa.proto"], &["src/"])?;
    Ok(())
}
