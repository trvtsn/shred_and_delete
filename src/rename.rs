use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::rngs::OsRng;
use walkdir::WalkDir;
use crate::PathType;
use crate::Query;
use crate::utils::is_dir_empty;

pub fn generate_random_name(extension: &str) -> String {
    let rand_string: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(10) // Generate a 10-character alphanumeric string
        .map(char::from)
        .collect();
    
    if extension.is_empty() {
        rand_string
    } else {
        format!("{}.{}", rand_string, extension)
    }
}

pub fn rename_to_random(query: &Query) -> io::Result<PathBuf> {
    if query.path_type == &PathType::Directory && !is_dir_empty(query.path)? {
        rename_files_and_directories(query.path)
    } else {
        rename(query.path)
    }
}

fn rename(path: &Path) -> io::Result<PathBuf> {
    let parent_dir: &Path = path.parent().unwrap_or_else(|| Path::new(""));
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let new_name = generate_random_name(extension);
    let final_path = parent_dir.join(new_name);

    fs::rename(path, &final_path)?;

    Ok(final_path)
}

fn rename_files_and_directories(dir_path: &Path) -> io::Result<PathBuf> {
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
        rename(&file)?;
    }

    // Then rename the directories, starting with the deepest level
    directories.sort_by_key(|dir| dir.components().count());
    directories.reverse();
    for directory in directories.iter_mut() {
        rename(directory)?;
    }

    // Finally, rename the root directory
    rename(dir_path)
}
