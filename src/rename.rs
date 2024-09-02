use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::rngs::OsRng;
use walkdir::WalkDir;
use crate::utils::is_dir_empty;

pub fn generate_random_name(extension: &str) -> String {
    let rand_string: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(10) // Generate a 10-character alphanumeric string
        .map(char::from)
        .collect();
    
    if extension.is_empty() {
        rand_string
    } 
    else {
        format!("{}.{}", rand_string, extension)
    }
}

pub fn rename_path_to_random<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();

    if path.is_dir() {
        if is_dir_empty(path)? {
            return rename_directory(path);
        } else {
            return rename_files_and_directories(path);
        }
    }
    else {
        rename_file(path)
    }
}

fn rename_file<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let parent_dir = path.parent().unwrap_or_else(|| Path::new(""));

    let new_name = generate_random_name(extension);
    let new_path = parent_dir.join(new_name);

    fs::rename(path, &new_path)?;

    Ok(new_path)
}

fn rename_directory<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let parent_dir = path.parent().unwrap_or_else(|| Path::new(""));
    
    let new_name = generate_random_name("");
    let new_path = parent_dir.join(new_name);

    fs::rename(path, &new_path)?;

    Ok(new_path)
}

fn rename_files_and_directories<P: AsRef<Path>>(dir_path: P) -> io::Result<PathBuf> {
    let dir_path = dir_path.as_ref();
    let mut files = Vec::new();
    let mut directories = Vec::new();
    
    for entry in WalkDir::new(dir_path).min_depth(1).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path().to_path_buf();
        if entry_path.is_file() {
            files.push(entry_path);
        } else if entry_path.is_dir() {
            directories.push(entry_path);
        }
    }

    // Do a rename on files first
    for file in files {
        rename_file(&file)?;
    }

    // Then rename the directories, starting with the deepest level
    directories.sort_by_key(|dir| dir.components().count());
    directories.reverse();
    for directory in directories {
        rename_directory(directory)?;
    }

    // Finally, rename the root directory
    rename_directory(dir_path)
}