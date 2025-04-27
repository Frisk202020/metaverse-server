use crate::file::File;
use std::str::Split;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String) -> Self { Self {name, directories: Vec::new(), files: Vec::new()} }
    fn add_dir(&mut self, d: Directory) { self.directories.push(d); }
    fn add_file(&mut self, f: File) {self.files.push(f); }

    fn build_dir(name: String, it: &mut Split<'_, &str>) -> Directory {
        let mut dir = Directory::new(name);
        loop {
            let line = if let Some(str) = it.next() { str.replace("---", "") } else { return dir };
            if line == "]" { return dir }
            else {
                if let Ok((name, extention, path)) = scan_fmt::scan_fmt!(&line, "{}:({}, {})", String, String, String) {
                    dir.add_file(File::new(name, extention, path));
                } else if let Ok(name) = scan_fmt::scan_fmt!(&line, "{}[", String) {
                    dir.add_dir(Directory::build_dir(name, it));
                } else {
                    panic!("Error parsing the line: {line}")
                }   
            }
        }
    }

    pub fn parse_file(file: String) -> Directory {
        let mut it = file.split("\n"); 
        let line = it.next().unwrap();
        Directory::build_dir(scan_fmt::scan_fmt!(line, "{}[", String).unwrap(), &mut it)    
    }
}