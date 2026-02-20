use anyhow::{anyhow, Context};
use std::fmt;
use std::io;

fn main() {
    divan::main();
}

// --- Error creation benchmarks ---

#[divan::bench]
fn create_from_msg() -> anyhow::Error {
    anyhow!("an error message")
}

#[divan::bench]
fn create_from_msg_with_fmt() -> anyhow::Error {
    let val = 42;
    anyhow!("an error message with value: {}", val)
}

#[divan::bench]
fn create_from_io_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::new(io_err)
}

#[divan::bench]
fn create_from_custom_error() -> anyhow::Error {
    anyhow::Error::new(CustomError)
}

// --- Context benchmarks ---

#[divan::bench]
fn context_on_io_error() -> anyhow::Result<()> {
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "oh no"));
    result.context("while performing an operation")
}

#[divan::bench]
fn context_with_lazy_message() -> anyhow::Result<()> {
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "oh no"));
    result.with_context(|| format!("while performing operation {}", 42))
}

#[divan::bench]
fn context_chained() -> anyhow::Error {
    anyhow!("root cause")
        .context("first context")
        .context("second context")
        .context("third context")
}

// --- Display and formatting benchmarks ---

#[divan::bench]
fn display_simple(bencher: divan::Bencher) {
    let err = anyhow!("an error message");
    bencher.bench(|| format!("{}", &err))
}

#[divan::bench]
fn display_alternate(bencher: divan::Bencher) {
    let err = anyhow!("root cause").context("outer context");
    bencher.bench(|| format!("{:#}", &err))
}

#[divan::bench]
fn debug_format(bencher: divan::Bencher) {
    let err = anyhow!("root cause").context("outer context");
    bencher.bench(|| format!("{:?}", &err))
}

// --- Downcast benchmarks ---

#[divan::bench]
fn downcast_ref_hit(bencher: divan::Bencher) {
    let err = anyhow::Error::new(CustomError);
    bencher.bench(|| err.downcast_ref::<CustomError>().is_some())
}

#[divan::bench]
fn downcast_ref_miss(bencher: divan::Bencher) {
    let err = anyhow::Error::new(CustomError);
    bencher.bench(|| err.downcast_ref::<io::Error>().is_some())
}

#[divan::bench]
fn downcast_through_context(bencher: divan::Bencher) {
    let err = anyhow::Error::new(CustomError).context("added context");
    bencher.bench(|| err.downcast_ref::<CustomError>().is_some())
}

// --- Chain iteration benchmark ---

#[divan::bench]
fn chain_short(bencher: divan::Bencher) {
    let err = anyhow::Error::new(CustomError).context("context");
    bencher.bench(|| err.chain().count())
}

#[divan::bench]
fn chain_long(bencher: divan::Bencher) {
    let err = anyhow!("root")
        .context("c1")
        .context("c2")
        .context("c3")
        .context("c4")
        .context("c5");
    bencher.bench(|| err.chain().count())
}

// --- Helper types ---

#[derive(Debug)]
struct CustomError;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("custom error")
    }
}

impl std::error::Error for CustomError {}
