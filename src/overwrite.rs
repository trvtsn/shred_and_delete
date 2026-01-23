use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Result};
use std::path::Path;
use rand::rngs::OsRng;
use rand::RngCore;
use walkdir::WalkDir;
use crate::{PathType, Query};
use crate::utils::is_dir_empty;

pub fn overwrite_with_random_data(query: &Query) -> Result<()> {
    match query.path_type {
        PathType::Directory => {
            if is_dir_empty(query.path)? {
                return Ok(());
            }

            // WalkDir crate allows us to traverse the directory recursively
            for entry in WalkDir::new(query.path).min_depth(1) {
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
        PathType::File => {
            match write_random_data(query.path) {
                Ok(_) => println!("Successfully shredded: {:?}", query.path.to_str().unwrap_or("Invalid UTF-8 path")),
                Err(_) => println!("Failed to shred: {:?}", query.path.to_str().unwrap_or("Invalid UTF-8 path")),
            };

            Ok(())
        }
    }
}

fn write_random_data(path: &Path) -> Result<()> {
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
