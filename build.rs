use std::fmt::Write;
use std::fs::{self, File};
use std::io::{self, Write as IoWrite};
use std::path::Path;

const PROBLEMS_DIR: &'static str = "src/problems";

fn main() {
    let dest_path = Path::new(PROBLEMS_DIR).join("mod.rs");
    let mut problems_mod = File::create(&dest_path).unwrap();

    let contents = mod_contents().unwrap();
    problems_mod.write_all(contents.as_bytes()).unwrap();
}

fn mod_contents() -> io::Result<String> {
    let mut contents = String::new();
    for entry in fs::read_dir(PROBLEMS_DIR)? {
        let entry = entry?;
        let mod_path = entry.path();
        let mod_name = mod_path.file_stem().unwrap().to_str().unwrap();

        if mod_name != "mod" {
            writeln!(&mut contents, "pub mod {};", mod_name).unwrap();
        }
    }

    Ok(contents)
}
