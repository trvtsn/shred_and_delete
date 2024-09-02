mod overwrite;
mod rename;
mod utils;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn shred_path<P: AsRef<Path>>(path: P, verb: &str) -> io::Result<()> 
{
    let new_path = rename::rename_path_to_random(&path)?;
    overwrite::overwrite_path_with_random_data(&new_path)?;

    if verb == "trash" {
        match trash::delete(new_path) {
            Ok(_) => println!("Path successfully trashed."),
            Err(e) => eprintln!("Error trashing path: {}", e),
        }
    }
    else if verb == "delete" {
        if new_path.is_dir() {
            match fs::remove_dir_all(new_path) {
                Ok(_) => println!("Path successfully deleted."),
                Err(e) => eprintln!("Error deleting path: {}", e),
            }
        } else if new_path.is_file() {
            match fs::remove_file(&new_path) {
                Ok(_) => println!("Path successfully deleted."),
                Err(e) => eprintln!("Error deleting path: {}", e),
            }
        }
    }
    else if verb == "default" {
        return Ok(())
    }

    Ok(())
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    eprintln!("Path: {}", path);
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    else if args.contains(&"--trash".to_string()) {
        match shred_path(path, "trash") {
            Ok(_) => println!("Files successfully shredded, renamed, and trashed."),
            Err(e) => eprintln!("Error shredding files: {}", e),
        }
    }
    else if args.contains(&"--delete".to_string()) {
        match shred_path(path, "delete") {
            Ok(_) => println!("Files successfully shredded, renamed, and deleted."),
            Err(e) => eprintln!("Error shredding files: {}", e),
        }
    }
    else {
        match shred_path(path, "default") {
            Ok(_) => println!("Files successfully shredded and renamed."),
            Err(e) => eprintln!("Error shredding files: {}", e),
        }
    }
}
