use winresource;
use std::io;

fn main() -> io::Result<()> 
{
    if cfg!(target_os = "windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .set_output_directory(".")
            .set("FileVersion", "1.0.0")
            .set("OriginalFilename", "shred_and_delete.exe")
            .set("FileDescription", "A rust-based file shredder")
            .set("LegalCopyright", "Copyright © 2024 Tarvi Tasane");
        res.compile()?;
    }

    Ok(())
}