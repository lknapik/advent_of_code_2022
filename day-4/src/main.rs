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

    let mut total_count = 0;
    let mut total_count_2 = 0;

    //for every line in the file
    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

        let mut split = line.split(",");

        let first_group = split.next().unwrap();
        let second_group = split.next().unwrap();

        split = first_group.split("-");

        let first_group_start = split.next().unwrap().parse::<i32>().expect("Number");
        let first_group_end = split.next().unwrap().parse::<i32>().expect("Number");

        split = second_group.split("-");

        let second_group_start = split.next().unwrap().parse::<i32>().expect("Number");
        let second_group_end = split.next().unwrap().parse::<i32>().expect("Number");

        if (first_group_start >= second_group_start && first_group_end <= second_group_end) || (second_group_start >= first_group_start && second_group_end <= first_group_end) {
            total_count += 1;
        }

        if (first_group_start >= second_group_start && first_group_start <= second_group_end) || (second_group_start >= first_group_start && second_group_start <= first_group_end) {
            total_count_2 += 1;
        }

    }
    println!("PART 1 | {}", total_count);
    println!("PART 2 | {}", total_count_2);
}
