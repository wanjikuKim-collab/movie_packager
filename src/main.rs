use clap::{App, Arg};
use std::process::Command;


fn main() {

    //Defining command- line arguments using clap
    let matches = App::new("Movie Packager")
        .version("1.0")
        .author("Faith Kimani")
        .about("Packaging movies using ffmpeg")
        .arg(
            Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input movie files (comma-separated)")
            .required(true),
        )
        .arg(
            Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets the output packaged movie file")
            .required(true),
        )
        .get_matches();

    //Extracting values of parsed arguments
    let input_files = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output"). unwrap();

   

}
