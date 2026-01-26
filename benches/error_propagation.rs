use anyhow::{Context, Result};
use std::io;

fn main() {
    divan::main();
}

fn fallible_operation() -> io::Result<String> {
    Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
}

fn nested_function_level1() -> Result<String> {
    fallible_operation().context("level1 failed")
}

fn nested_function_level2() -> Result<String> {
    nested_function_level1().context("level2 failed")
}

fn nested_function_level3() -> Result<String> {
    nested_function_level2().context("level3 failed")
}

#[divan::bench]
fn propagate_with_question_mark() -> Result<()> {
    let _ = fallible_operation()?;
    Ok(())
}

#[divan::bench]
fn propagate_with_single_context() -> Result<()> {
    let _ = fallible_operation().context("operation failed")?;
    Ok(())
}

#[divan::bench]
fn propagate_through_nested_calls() -> Result<()> {
    let _ = nested_function_level3()?;
    Ok(())
}

#[divan::bench]
fn error_to_string() -> String {
    let result: Result<()> = nested_function_level3().map(|_| ());
    match result {
        Err(e) => e.to_string(),
        Ok(_) => String::new(),
    }
}

#[divan::bench]
fn error_debug_format() -> String {
    let result: Result<()> = nested_function_level3().map(|_| ());
    match result {
        Err(e) => format!("{:?}", e),
        Ok(_) => String::new(),
    }
}

#[divan::bench]
fn error_chain_iteration() -> usize {
    let result: Result<()> = nested_function_level3().map(|_| ());
    match result {
        Err(e) => e.chain().count(),
        Ok(_) => 0,
    }
}
