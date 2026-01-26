use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn create_simple_error() -> anyhow::Error {
    anyhow!("simple error message")
}

#[divan::bench]
fn create_formatted_error() -> anyhow::Error {
    anyhow!("error with value: {}", 42)
}

#[divan::bench]
fn create_from_std_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::from(io_err)
}

#[divan::bench]
fn create_with_context() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).context("failed to read config file")?;
    Ok(())
}

#[divan::bench]
fn create_with_lazy_context() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).with_context(|| format!("failed to read file: {}", "config.json"))?;
    Ok(())
}

#[divan::bench]
fn downcast_error() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = anyhow::Error::from(io_err);
    
    divan::black_box(err.downcast_ref::<io::Error>());
    Ok(())
}

#[divan::bench]
fn chain_multiple_contexts() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err)
        .context("failed to read config")
        .context("failed to initialize application")?;
    Ok(())
}
