mod cli;
mod overwrite;
mod rename;
mod utils;

use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;
use crate::cli::Args;
use crate::cli::Method;

pub struct Query<'a> {
    path: &'a Path,
    method: &'a Method,
    path_type: &'a PathType
}

#[derive(PartialEq, Debug)]
pub enum PathType {
    File,
    Directory
}

impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PathType::File => "File",
            PathType::Directory => "Directory",
        };
        write!(f, "{s}")
    }
}

fn start_shred(query: &Query) -> io::Result<()> 
{
    let path_type = query.path_type;
    let new_path = rename::rename_to_random(&query)?;
    overwrite::overwrite_with_random_data(&query)?;

    match query.method {
        Method::Trash => {
            match trash::delete(new_path) {
                Ok(_) => println!("{path_type} successfully trashed."),
                Err(e) => eprintln!("Error trashing {path_type}: {e}"),
            }
        }
        Method::Delete => {
            if new_path.is_dir() {
                match fs::remove_dir_all(new_path) {
                    Ok(_) => println!("{path_type} successfully deleted."),
                    Err(e) => eprintln!("Error deleting {path_type}: {e}"),
                }
            } else if new_path.is_file() {
                match fs::remove_file(&new_path) {
                    Ok(_) => println!("{path_type} successfully deleted."),
                    Err(e) => eprintln!("Error deleting {path_type}: {e}"),
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = cli::Args::parse();
    let Args { ref method, ref path } = args;
    eprintln!("Path: {}", path);

    let path = Path::new(path);
    let path_type = if path.is_dir() { &PathType::Directory } else if path.is_file() { &PathType::File } else { return Ok(()) };
    let query = Query { path, method, path_type};

    match start_shred(&query) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
