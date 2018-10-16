extern crate clap;
use clap::{Arg, App};

//use std::error::Error;
//use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};

extern crate quick_xml;
/*
use quick_xml::Reader;
use quick_xml::events::Event;
*/

fn main() {
    let matches = App::new("xsd2ebnf")
                          .version("0.1")
                          .author("Shiva <shiv@shiv.me>")
                          .about("Converts XSD to EBNF")
                          .arg(Arg::with_name("output")
                               .short("o")
                               .long("output")
                               .value_name("FILE")
                               .help("Output ebnf filename")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Input xsd file")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let ifile = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", ifile);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // Use output file if available, otherwise default to stdout
    let ofilename = matches.value_of("output");
    let _out_writer = match ofilename {
        Some(x) => {
            let path = Path::new(x);
            println!("Using output file: {}", x);
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => {
            println!("Using stdout");
            Box::new(io::stdout()) as Box<Write>
        }
    };

    // read input file
    /*
    let mut reader = Reader::from_file(ifile);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                out_writer.write(e.name());


                    /*
                match e.name() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value)
                                        .collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    _ => (),
                }
                    */
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
    */
}
