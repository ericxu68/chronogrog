use std::io::{BufReader, Read, Write};
use std::fs::File;

extern crate chronogrog;
use chronogrog::ProductionSchedule;

#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let app_name = format!("{}", env!("CARGO_PKG_NAME"));
    let app_description = format!("{}", env!("CARGO_PKG_DESCRIPTION"));
    let authors = format!("{}", env!("CARGO_PKG_AUTHORS"));

    let matches = App::new(app_name)
      .version(crate_version!())
      .about(&app_description[..])
      .author(&authors[..])
      .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("Specify an input file to read from. Defaults to standard input.")
                .takes_value(true),
      )
      .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Specify an output file to write to. Defaults to standard output.")
                .takes_value(true),
      )
      .get_matches();

    let input_file: Box<dyn Read> = match matches.value_of("input") {
        Some(in_file) => match File::open(in_file) {
                Ok(f) => Box::new(f),
                Err(e) => {
                    panic!(format!("{}: {}", e, in_file));
                }
        },
        None => Box::new(std::io::stdin())
    };

    let mut buf_reader = BufReader::new(input_file);
    let mut json_data: String = String::new();
    buf_reader.read_to_string(&mut json_data).unwrap();

    let production_schedule: ProductionSchedule = ProductionSchedule::new(&json_data[..]);

    let output_file: Box<dyn Write> = match matches.value_of("output") {
        Some(out_file) => match File::create(out_file) {
            Ok(f) => Box::new(f),
            Err(e) => panic!(format!("{}: {}", e, out_file))
        },
        None => Box::new(std::io::stdout())
    };

    match production_schedule.write_pla_file(output_file) {
        Ok(_x) => _x,
        Err(e) => panic!("{}", e)
    }
}
