use handlebars::Handlebars;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::error::Error;
use std::path::Path;

use super::endpoints::ToDescriptor;

pub fn parse_file_template<T>(path: &'static str, data: &T) -> String
where
    T: ToDescriptor,
{
    let mut reg = Handlebars::new();
    reg.register_template_file("file", path).unwrap();

    reg.render("file", &data.descriptor()).unwrap()
}

pub fn write_file<T>(data: &T, content: String)
where
    T: ToDescriptor
{
    let path = Path::new("./lol-maintainer/my_file.rs");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
