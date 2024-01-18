extern crate clap;// parses CLI arguments
use clap::{Arg, Command as ClapCommand};
use std::{process::{Command}, fs, path::Path};

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

    //output directory path
    let output_dir = "src/assets/outputs";
    
    // Output directory (with error handling)
    match fs::create_dir_all(output_dir) {
        Ok(()) => println!("Output directory created successfully"),
        Err(error) => {
            eprintln!("Error creating output directory: {}", error);
            // Handle the error appropriately, e.g., exit the program
        }
    }

    // Parse command-line arguments
    let matches = app.get_matches();

    // Extract values of parsed arguments
    let input_files:&String = matches.get_one("input").unwrap();
    let output_file = match matches.get_one("output") {
        Some(output_path) => {
            if !Path::new(&output_dir).file_name() {
                eprintln!("Error: Invalid output file path: {}", output_path);
                // Handle the error appropriately
            } else {
                // Use the validated output_path here
            }
        }
        None => {
            eprintln!("Error: Please provide an output file path using the --output flag.");
            // Handle the error appropriately, e.g., exit the program or prompt for input
        }
    }

    let out_path = format!("{}/stream1", output_dir);
    //Converting input file string to vector of input files(the .ts files)
    let input_files: Vec<&str> = input_files.split(',').collect();

    //Using ffmpeg to concatenate input files into the output file
    let mut cmd = Command::new("ffmpeg"); 
    cmd.arg("-i")
    .arg(input_files[0]) // Take first input file
    .arg("-c:v") 
    .arg("libx264")
    .arg("-hls_time")  
    .arg("10")
    .arg("-hls_list_size") 
    .arg("0")
    .arg("-f")
    .arg("hls")
    .arg(out_path); // HLS playlist output

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

