//https://adventofcode.com/2022/day/1
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    //initiate vector and sum
    let mut elf_totals = Vec::new();
    let mut sum: i32 = 0;

    //for every line in the file
    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

        //if not blank line, must be same elf
        if line != ""{
            //convert line string to i32 and add to sum
            sum += line.parse::<i32>().unwrap();
        } else {
            //push sum to vec, reset sum
            elf_totals.push(sum);
            sum = 0;
        }
    }
    //sort and reverse list, now largest -> smallest
    elf_totals.sort();
    elf_totals.reverse();
    
    println!("Top Elf = {:?}", elf_totals[0]);

    println!("Top 3 Combined = {:?}", elf_totals[0]+elf_totals[1]+elf_totals[2]);

}
