#[macro_use]
extern crate clap;

use clap::App;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let source_file = matches.value_of("input").unwrap();
    println!("Reading JSON file: {}", source_file);
    let file = match File::open(source_file) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open input file!");
            println!("{}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let json: Value = match serde_json::from_reader(reader) {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to parse source file!");
            println!("{}", e);
            return;
        }
    };

    let output_file = matches.value_of("output").unwrap();
    println!("Writing output file: {}", output_file);
    let out = match File::create(output_file) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open output file!");
            println!("{}", e);
            return;
        }
    };

    let writer = BufWriter::new(out);
    match serde_json::to_writer_pretty(writer, &json) {
        Ok(()) => {}
        Err(e) => {
            println!("Error writing JSON to output file!");
            println!("{}", e);
            return;
        }
    }
}
