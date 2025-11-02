use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader};

pub fn open_file(path: &str) -> io::Result<File> {
    File::open(path)
}

pub fn read_file(path: &str) -> io::Result<String> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn append_file(path: &str, contents: &str, append: bool) -> io::Result<()> {
    if append {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        f.write_all(contents.as_bytes())?;
    } else {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        f.write_all(contents.as_bytes())?;
    }
    Ok(())
}
