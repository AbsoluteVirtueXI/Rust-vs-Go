use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io;
use std::process;
use thiserror::Error;

const BINARY_NAME: &str = "rust-cat";
const NB_ARGS: usize = 1;

#[derive(Error, Debug)]
enum CatError {
    #[error("usage: {} filename", BINARY_NAME)]
    BadUsage,
    #[error("file {0} is empty")]
    EmptyFile(String),
    #[error(transparent)]
    IOError(#[from] io::Error),
}

fn main() {
    let filename = match extract_filename() {
        Ok(filename) => filename,
        Err(e) => {
            eprintln!("{e}");
            process::exit(-1)
        }
    };

    let content = match read_file(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(-1);
        }
    };

    print!("{content}");
}

fn extract_filename() -> Result<String, CatError> {
    match env::args().skip(1).count().cmp(&NB_ARGS) {
        Ordering::Equal => Ok(env::args().skip(1).take(NB_ARGS).collect::<String>()),
        _ => Err(CatError::BadUsage),
    }
}

fn read_file(filename: &str) -> Result<String, CatError> {
    let content = fs::read_to_string(filename)?;

    if !content.is_empty() {
        Ok(content)
    } else {
        Err(CatError::EmptyFile(filename.to_owned()))
    }
}
