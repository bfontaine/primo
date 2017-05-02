extern crate clap;

mod primo;
use clap::{App, Arg};

static VERSION : &str = "0.1.0";

fn main() {
    let matches = App::new("primo")
                    .version(VERSION)
                    .author("Baptiste Fontaine <b@ptistefontaine.fr>")
                    .about("Sort stuff")
                    .arg(Arg::with_name("FILE")
                                .help("Input file")
                              //.default_value("-")
                                .index(1))
                    .get_matches();

    //let input = matches.value_of("FILE").unwrap();
    // TODO
}
