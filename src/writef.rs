use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use phrase::DwPhrase;

pub struct FileOut {
    path: String,
    contents: Vec<String>,
}

impl FileOut {
    pub fn new(path: String,  contents: Vec<String>) -> FileOut {
        FileOut { path, contents}
    }

    pub fn append_str(&mut self, data: &str, separator: &str)  {
        self.contents.push(data.to_string());
        self.contents.push(separator.to_string())   
    }

    pub fn write(&self, phrase: &DwPhrase) {
        let path = Path::new(&self.path);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Could not create: {} {}", display, why.description()),
            Ok(file) => file,
        };

        let output: String = self.contents
                                 .iter()
                                 .map(|s| format!("{}\n", s))
                                 .collect();

        match file.write_all(output.as_bytes()) {
            Err(why) => panic!("Could not write: {} {}", display, why.description()),
            Ok(_) => println!("\nContents written to file."),
        }
    }

}