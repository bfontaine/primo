extern crate clap;
extern crate primo;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use clap::{App,Arg};

static VERSION : &str = "0.0.1";

fn read_lines(filename : &str) -> Vec<String> {
    let mut lines = Vec::new();

    match filename {
        // stdin
        "-" => {
            let reader = io::stdin();
            for line in BufReader::new(reader.lock()).lines() {
                lines.push(line.unwrap());
            };
        },
        _ => {
            let path = Path::new(filename);

            if !path.exists() {
                panic!("File '{}' doesn't exist", path.display());
            }

            if !path.is_file() {
                panic!("'{}' is not a regular file", path.display());
            }

            match File::open(path) {
                Err(why) => panic!("Can't open '{}': {}", path.display(), why.description()),
                Ok(file) => for line in BufReader::new(file).lines() {
                    lines.push(line.unwrap());
                },
            }
        },
    }

    lines
}

fn main() {
    let matches = App::new("primo")
                    .version(VERSION)
                    .author("Baptiste Fontaine <b@ptistefontaine.fr>")
                    .about("Sort lines of text files")
                    .arg(Arg::with_name("FILE")
                               .help("Optional input file")
                               .index(1))
                    .get_matches();

    let filename = matches.value_of("FILE").unwrap_or("-");

    let mut lines = read_lines(filename);

    primo::sort_vec(&mut lines);

    for line in lines {
        println!("{}", line);
    }
}
