fn main() {
    divan::main();
}

use anyhow::{anyhow, Context, Result};
use std::io;

#[divan::bench]
fn create_error_from_message() {
    let _: anyhow::Error = anyhow!("error message");
}

#[divan::bench]
fn create_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let _: anyhow::Error = io_error.into();
}

#[divan::bench]
fn error_with_context() -> Result<()> {
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read config file")
}

#[divan::bench]
fn error_with_multiple_context() -> Result<()> {
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read config file")
        .context("Application initialization failed")
}

#[divan::bench]
fn error_downcast() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    divan::black_box(error.downcast_ref::<io::Error>());
}

#[divan::bench]
fn error_chain_iteration() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found")
        .into();
    let error = anyhow::Error::msg("wrapper").context(error);
    
    for cause in error.chain() {
        divan::black_box(cause);
    }
}

#[divan::bench]
fn format_error() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found")
        .into();
    let formatted = format!("{}", error);
    divan::black_box(formatted);
}

#[divan::bench]
fn format_error_debug() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found")
        .into();
    let formatted = format!("{:?}", error);
    divan::black_box(formatted);
}
