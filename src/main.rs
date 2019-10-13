use chip8_rom_to_opcode::RomData;
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

    if let Ok(bytes) = read_file(file) {
        disassemble(&bytes);
    } else {
        eprintln!("Unable to disassemble open file: {}", file);
    }
}

fn read_file(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut f = File::open(&filename)?;
    println!("Disassembling file: {} \n", filename);
    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data)?;
    Ok(data)
}

// disassemble and print the contents to the terminal
fn disassemble(data: &[u8]) {
    let opcodes = RomData::new(data).get_all_opcodes();
    opcodes.iter().for_each(|opcode| println!("{}", opcode));
}
