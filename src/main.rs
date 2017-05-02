extern crate clap;

mod primo;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use clap::App;

static VERSION : &str = "0.1.0";

fn main() {
    let matches = App::new("primo")
                    .version(VERSION)
                    .author("Baptiste Fontaine <b@ptistefontaine.fr>")
                    .about("Sort stuff")
                    .args_from_usage("<FILE> 'Input file'")
                    .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let path = Path::new(filename);

    if !path.exists() {
        panic!("File '{}' doesn't exist", path.display());
    }

    if !path.is_file() {
        panic!("'{}' is not a regular file", path.display());
    }

    let reader = match File::open(path) {
        Err(why) => panic!("Can't open '{}': {}", path.display(), why.description()),
        Ok(file) => BufReader::new(file),
    };

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap())
    }

    primo::sort_vec(&mut lines);

    for line in lines {
        println!("{}", line);
    }
}
