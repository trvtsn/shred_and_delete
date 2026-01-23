use std::fs;
use std::io;
use std::path::Path;

pub fn is_dir_empty(path: &Path) -> io::Result<bool> {
    fs::read_dir(path).map(|mut e| e.next().is_none())
}
