extern crate clap;
use clap::{Arg, Command as ClapCommand};
use std::process::{Command, exit};

fn main() {
    // Create the App builder
    let app = ClapCommand::new("Movie Packager")
        .version("1.0")
        .author("Faith Kimani")
        .about("Packaging movies using ffmpeg")
        // Define command-line arguments
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input movie files (comma-separated)")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output packaged movie file")
                .required(true),
        );

    // Parse command-line arguments
    let matches = app.get_matches();

    // Extract values of parsed arguments
    let input_files:&String = matches.get_one("input").unwrap();
    let output_file:&String = matches.get_one("output").unwrap();

    //Converting input file string to vector of input files
    let input_files: Vec<&str> = input_files.split(',').collect();

    

}

