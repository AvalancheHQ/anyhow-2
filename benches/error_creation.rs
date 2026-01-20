fn main() {
    divan::main();
}

use std::io;

#[divan::bench]
fn error_from_message() -> anyhow::Error {
    anyhow::anyhow!("operation failed")
}

#[divan::bench]
fn error_from_formatted() -> anyhow::Error {
    anyhow::anyhow!("operation {} failed with code {}", "read", 42)
}

#[divan::bench]
fn error_from_io_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::from(io_err)
}

#[divan::bench]
fn error_with_context() -> anyhow::Result<()> {
    use anyhow::Context;
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read configuration")
}

#[divan::bench]
fn error_with_lazy_context() -> anyhow::Result<()> {
    use anyhow::Context;
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .with_context(|| format!("Failed to read file at path: {}", "/config.json"))
}

#[divan::bench]
fn error_downcast_success(bencher: divan::Bencher) {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    bencher.bench(|| {
        error.downcast_ref::<io::Error>()
    });
}

#[divan::bench]
fn error_chain_iteration(bencher: divan::Bencher) {
    use anyhow::Context;
    let error = Err::<(), _>(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read config")
        .context("Failed to initialize app")
        .unwrap_err();
    
    bencher.bench(|| {
        error.chain().count()
    });
}
