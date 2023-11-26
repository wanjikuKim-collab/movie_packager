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

    //Converting input file string to vector of input files
    let input_files: Vec<&str> = input_files.split(',').collect();

    //Using ffmpeg to concatenate input files into the output file
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").args(input_files).arg("-filter_complex").arg("concat=n=-2:v=1:a=1").arg(output_file);


    match cmd.status(){
        Ok(exit_status)=>{
            if exit_status.success(){
                println!("Movies packaged successfully into: {}", output_file);
            } else {
                eprintln!("Error: Failed to create the package.");
            }
        }
        Err(e)=> eprintln!("Error: Failed to execute ffmpeg command: {}", e)
    }

}
