use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let matches = App::new("chip8-dasm")
        .version(clap::crate_version!())
        .about("Disassembler for chip-8 binaries")
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("file")
                .value_name("binary")
                .help("The file to disassemble")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap_or_else(|| {
        println!("No binary file provided");
        process::exit(1);
    });

    if let Err(error) = read_file(file) {
        eprintln!("Unable to disassemble {} | {}", file, error.to_string());
    }
}

fn read_file(filename: &str) -> std::io::Result<()> {
    let mut f = File::open(&filename)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    println!("file contents\n {}", data);
    Ok(())
}
