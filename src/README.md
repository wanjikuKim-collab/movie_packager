clap is a library which provides the ability to parse command line options.
ffmpeg is a universal media converter

// cli command: 
ffmpeg -i input.mp4 -profile:v baseline -level 3.0 -s 640x360 -start_number 0 -hls_time 10 -hls_list_size 0 -f hls index.m3u8