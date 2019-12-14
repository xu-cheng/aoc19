pub use anyhow::{anyhow, bail, Context, Error, Result};
pub use itertools::{self, Itertools};
pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::io::{self, BufReader};
pub use std::path::PathBuf;

pub fn open_input(file_name: &str) -> Result<BufReader<File>> {
    let file_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "data", file_name]
        .iter()
        .collect();
    let reader = BufReader::new(File::open(file_path)?);
    Ok(reader)
}

pub mod computer;
