use std::io;
use std::path::Path;

pub fn resource_from_file<P: AsRef<Path>>(resource: P) -> Result<String, io::Error> {
    use std::path::PathBuf;

    let mut path = PathBuf::from("resources");
    path.push(resource);

    string_from_file(path)
}

fn string_from_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = try!(File::open(path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}
