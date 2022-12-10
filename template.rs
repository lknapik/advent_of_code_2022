//https://adventofcode.com/2022/day/
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);


    //for every line in the file
    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

    }

}
