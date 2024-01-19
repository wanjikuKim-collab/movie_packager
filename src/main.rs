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
        
    // Parse command-line arguments
    let matches = app.get_matches();
    
    // Extracting parsed input values and convert to vector files
    let input_files: Vec<&str> = matches.get_one::<String>("input").unwrap().split(',').collect();

    for input_file in input_files{
        //Error handling for canonical path
        let canonical_input_path = match fs::canonicalize(input_file) {
            Ok(canonical_path) =>  canonical_path ,
            Err(e)=>{
                eprintln!("Error resolving input file path: {}", e);
                continue;            
            }
        };

        //Extracting base filename without extension
        let filename = canonical_input_path.file_name().unwrap().to_str().unwrap();
        let base_filename = filename.split('.').next().unwrap();
        
        //Creating output folder for the respective input file
        let output_folder = format!("assets/outputs/{}", base_filename);
        fs::create_dir_all(&output_folder).unwrap_or_else(|e|{
           eprintln!("Failed to create output folder: {}",e);
           std::process::exit(1); 
        });

        //Constructs full output path within the folder
        let output_file = format!("{}/{}.m3u8", output_folder, base_filename);
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

    // // Output directory creation with error handling
    // let output_dir = "assets/outputs";
    // match fs::create_dir_all(output_dir) {
    //     Err(e) => {
    //       eprintln!("Failed to create output dir: {}", e);
    //       std::process::exit(1);
    //     }
    //     Ok(_) => { // dir created 
    //     }
    // }

  

    
    


}

