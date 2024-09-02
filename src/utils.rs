use std::fs;
use std::io;
use std::path::Path;

pub fn is_dir_empty<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let mut entries = fs::read_dir(path)?;
    Ok(entries.next().is_none())
}