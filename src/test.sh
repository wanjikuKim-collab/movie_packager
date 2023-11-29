#!/bin/bash

# Set input and output files
input_files="input1.mp4,input2.mp4"
output_file="output.mp4"

# Execute ffmpeg command
ffmpeg -i "$input_files" -filter_complex "concat=n=-2:v=1:a=1" "$output_file"
