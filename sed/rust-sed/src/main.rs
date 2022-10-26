use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;
use thiserror::Error;

const BINARY_NAME: &str = "rust-sed";
const NB_ARGS: usize = 3;

#[derive(Debug, Error)]
enum SedError {
    #[error("usage: {} word1 word2 filename", BINARY_NAME)]
    BadUsage,
    #[error(transparent)]
    IOError(#[from] io::Error),
}

fn main() {
    let args = match extract_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(-1)
        }
    };

    if let Err(e) = sed(&args[0], &args[1], &args[2]) {
        eprintln!("Error: {}", e);
        process::exit(-1)
    }
}

fn extract_args() -> Result<Vec<String>, SedError> {
    match env::args().skip(1).count().cmp(&NB_ARGS) {
        Ordering::Equal => Ok(env::args().skip(1).take(NB_ARGS).collect::<Vec<_>>()),
        _ => Err(SedError::BadUsage),
    }
}

fn sed(old: &str, new: &str, filename: &str) -> Result<usize, SedError> {
    let f = File::open(filename)?;

    let mut count = 0;

    for res in BufReader::new(f).lines() {
        let (_, line, nb) = match res {
            Ok(line) => process_line(old, new, &line),
            Err(e) => return Err(SedError::IOError(e)),
        };
        println!("{}", line);
        count += nb;
    }

    Ok(count)
}

fn process_line(old: &str, new: &str, line: &str) -> (bool, String, usize) {
    if !line.contains(old) {
        (false, line.to_owned(), 0)
    } else {
        (true, line.replace(old, new), line.matches(old).count())
    }
}
