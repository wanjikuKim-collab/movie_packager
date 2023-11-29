extern crate clap;
use clap::{Arg, Command};
use std::process::{exit};

fn main() {
    // Create the App builder
    let app = Command::new("Movie Packager")
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
    let output_file:&Str = matches.get_one("output").unwrap();

    // Convert input file string to vector of input files
    let input_files_vec: Vec<&str> = input_files.split(',').collect();

    // Simplify FFMPEG Command Creation
    let input_files_str = input_files_vec.join(" -i ");
    let ffmpeg_command = format!(
        "ffmpeg -i {} -filter_complex concat=n={}:v=1:a=1 -y {}",
        input_files_str,
        input_files_vec.len(),
        output_file
    );

    // Execute ffmpeg command using std::process::Command
    let status = Command::new("sh").arg("-c").arg(&ffmpeg_command).status().unwrap();

    // Handle FFMPEG Exit Status
    if status.success() {
        println!("Movies packaged successfully into: {}", output_file);
    } else {
        eprintln!("Error: Failed to create the package.");
        exit(1);
    }
}
