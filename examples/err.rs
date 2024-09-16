use std::fs;

use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Serialize error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Custom error: {0}")]
    Custom(String),
}

fn main() -> Result<(), anyhow::Error> {
    println!("Size of MyError is {}", std::mem::size_of::<MyError>());

    let filename = "non-existent-file.text";
    let _fd =
        fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
