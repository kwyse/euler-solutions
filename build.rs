use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut solutions_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/solutions/mod.rs")?;

    for entry in fs::read_dir("src/solutions")? {
        let entry = entry?;
        if let Some(file_name) = entry
            .path()
            .file_stem()
            .and_then(OsStr::to_str)
            .filter(|&file_name| file_name != "mod")
        {
            solutions_file.write_all(format!("pub mod {};\n", file_name).as_bytes())?;
        }
    }

    Ok(())
}
