mod cli;
mod overwrite;
mod rename;
mod utils;

use anyhow::anyhow;
use clap::Parser;
use std::fs;
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

fn start_shred(query: &Query) -> anyhow::Result<()> 
{
    let path_type = query.path_type;
    let new_path = rename::rename_to_random(&query)?;
    overwrite::overwrite_with_random_data(&new_path, path_type)?;

    match query.method {
        Method::Trash => {
            match trash::delete(new_path) {
                Ok(_) => eprintln!("{path_type} successfully trashed."),
                Err(e) => return Err(anyhow!("Error trashing {}: {e}", path_type.to_string().to_lowercase())),
            }
        }
        Method::Delete => {
            if new_path.is_dir() {
                match fs::remove_dir_all(new_path) {
                    Ok(_) => eprintln!("{path_type} successfully deleted."),
                    Err(e) => return Err(anyhow!("Error deleting {}: {e}", path_type.to_string().to_lowercase())),
                }
            } else if new_path.is_file() {
                match fs::remove_file(&new_path) {
                    Ok(_) => eprintln!("{path_type} successfully deleted."),
                    Err(e) => return Err(anyhow!("Error deleting {}: {e}", path_type.to_string().to_lowercase()))
                }
            } else {
                return Err(anyhow!("Error: Path is neither directory or file?"))
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let Args { ref method, ref path } = args;
    eprintln!("Path: {}", path);

    let path = Path::new(path);
    let path_type = if path.is_dir() { 
        &PathType::Directory 
    } else if path.is_file() { 
        &PathType::File 
    } else { 
        return Err(anyhow!("Error: Path is neither directory or file?"))
    };
    let query = Query { path, method, path_type};

    match start_shred(&query) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
