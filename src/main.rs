extern crate clap;// parses CLI arguments
use clap::{Arg, Command as ClapCommand};
use std::{process::{Command}, fs};

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
        
    
    // Output directory creation with error handling
    let output_dir = "assets/outputs";
    match fs::create_dir_all(output_dir) {
        Err(e) => {
          eprintln!("Failed to create output dir: {}", e);
          std::process::exit(1);
        }
        Ok(_) => { // dir created 
        }
    }

    // Parse command-line arguments
    let matches = app.get_matches();

    // Extract values of parsed arguments
    let input_files: Vec<&str> = matches.get_one::<String>("input").unwrap().split(',').collect();//extracting parsed input values and converting to vector files

    for input_file in input_files{
        //Error handling for canonical path
        let output_file = match fs::canonicalize(input_file) {
            Ok(canonical_path) => {
                let filename = canonical_path.file_name().unwrap().to_str().unwrap();
                format!("{}/{}.m3u8", output_dir, filename)
            },
            Err(e)=>{
                eprintln!("Error resolving input file path: {}", e);
                // Handle the error gracefully, e.g., skip this file, prompt, or terminate
                continue;            
            }
        };
        
        //Using ffmpeg to concatenate input files into the output file
        let mut cmd = Command::new("ffmpeg"); 
        cmd.arg("-i")
        .arg(input_file) 
        .arg("-c:v") 
        .arg("libx264")
        .arg("-hls_time")  
        .arg("10")
        .arg("-hls_list_size") 
        .arg("0")
        .arg("-f")
        .arg("hls")
        .arg(&output_file);
        
        // HLS playlist output
        match cmd.status() {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!("Successfully packaged {} to {}", input_file, output_file);
                } else {
                    eprintln!("Error creating output file {}: ffmpeg failed", output_file);
                }
            },
            Err(e) => eprintln!("Error executing ffmpeg command: {}", e)
        }
    }


}

