use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use phrase::DwPhrase;

pub struct FileOut {
    path: String,
    contents: String,
}

impl FileOut {
    pub fn new(path: String,  contents: String) -> FileOut {
        FileOut { path, contents}
    }

    pub fn append_str(&mut self, data: &str) {
        self.contents.push_str(&data)
    }

    pub fn write(&self) {
        let path = Path::new(&self.path);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Could not create: {} {}", display, why.description()),
            Ok(file) => file,
        };

        match file.write_all(self.contents.as_bytes()) {
            Err(why) => panic!("Could not write: {} {}", display, why.description()),
            Ok(_) => println!("\n\t\tContents written to file."),
        }
    }

}