// src/overwrite.rs

use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Result};
use std::path::Path;
use rand::rngs::OsRng;
use rand::RngCore;
use walkdir::WalkDir;
use crate::utils::is_dir_empty;

pub fn overwrite_path_with_random_data<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    if path.is_dir() {
        if is_dir_empty(path)? {
            return Ok(())
        } else {
            // WalkDir crate allows us to traverse the directory recursively
            for entry in WalkDir::new(path).min_depth(1) {
                match entry {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        if entry_path.is_file() {
                            match write_random_data(entry_path) {
                                Ok(_) => println!("Successfully shredded: {:?}", entry_path.to_str().unwrap_or("Invalid UTF-8 path")),
                                Err(_) => println!("Failed to shred: {:?}", entry_path.to_str().unwrap_or("Invalid UTF-8 path")),
                            };
                        }
                    }
                    Err(e) => eprintln!("Failed to read directory entry: {}", e),
                }
            }
            Ok(())
        }
    } 
    else {
        match write_random_data(path) {
            Ok(_) => println!("Successfully shredded: {:?}", path.to_str().unwrap_or("Invalid UTF-8 path")),
            Err(_) => println!("Failed to shred: {:?}", path.to_str().unwrap_or("Invalid UTF-8 path")),
        };

        Ok(())
    } 
}

pub fn write_random_data<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut file = File::options().write(true).open(path)?;

    let file_size = file.metadata()?.len();

    // These 2 lines overwrite the path with random data
    let mut buffer = vec![0u8; file_size as usize];
    OsRng.fill_bytes(&mut buffer);
    file.write_all(&buffer)?;

    // These 2 lines overwrite the random data with zeroes
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&vec![0u8; file_size as usize])?;

    file.flush()?;
    file.sync_all()?;
    
    Ok(())
}