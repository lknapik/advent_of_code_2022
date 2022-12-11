//https://adventofcode.com/2022/day/5
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use std::collections::VecDeque;



fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let mut supplies = vec![VecDeque::new(); 9];
    let mut building_supplies = true;
    
    //for every line in the file
    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");
        
        if line.contains(" 1   2   3"){
            building_supplies = false;
            continue;
        }
        if line == "" {
            continue;
        }

        if building_supplies == true {
            for (index, c) in line.chars().enumerate() {
                if index % 4 == 1 {
                    if c != ' ' {
                        supplies[index/4].push_front(c);
                    }
                }
            }
        } else {
            let mut split = line.split(" ");
            split.next();
            let move_number = split.next().unwrap().parse::<i32>().expect("Number");
            split.next();
            let start_stack = split.next().unwrap().parse::<usize>().expect("Number") - 1;
            split.next();
            let end_stack = split.next().unwrap().parse::<usize>().expect("Number") - 1;
            /* Uncomment to run part 1
            for _ in 0..move_number {
                let tmp = supplies[start_stack].pop_back().unwrap();
                supplies[end_stack].push_back(tmp);
            }
            */
            let len = supplies[start_stack].len();
            let tmp_buffer = &mut supplies[start_stack].split_off(len-move_number as usize);
            supplies[end_stack].append(tmp_buffer);
            tmp_buffer.clear();
        }
    }

    for mut stack in supplies {
        println!("{}", stack.pop_back().unwrap());
    }
}
