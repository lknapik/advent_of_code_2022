//https://adventofcode.com/2022/day/10
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    //open file handle
    let file = File::open("real.csv").expect("file not found");

    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let mut cycle_count: i32 = 1;
    let mut x: i32 = 1;
    let mut signal_str: i32 = 0;

    //for every line in the file
    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

        if line.contains("noop") {
            cycle_count += 1;
        } else if line.contains("addx") {
            cycle_count += 1;
            if cycle_count.rem_euclid(40) == 20 {
                signal_str += cycle_count * x;
            }
            let change: i32 = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            x += change;
            cycle_count += 1;
        }

        if cycle_count.rem_euclid(40) == 20 {
            signal_str += cycle_count * x;
        }
    }

    println!("Signal Str: {}", signal_str);
}
