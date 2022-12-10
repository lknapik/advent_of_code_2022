//https://adventofcode.com/2022/day/3
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn find_common_badge(first: &str, second: &str, third: &str) -> char {

    for c in first.chars(){
        if second.contains(c){
            if third.contains(c){
                return c;
            }
        }
    }

    return '0';
}

fn find_common_letter(first: &str, second: &str) -> char {

    for c in first.chars(){
        if second.contains(c){
            return c;
        }
    }

    return '0';
}

fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");

    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let mut total_score: i32 = 0;
    let mut total_score_2: i32 = 0;
    let mut str_list = Vec::new();

    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

        let len = line.len();

        let first_half = &line[0..len/2];
        let second_half = &line[len/2..len];

        let mut common_letter = find_common_letter(first_half, second_half) as i32;
        let mut common_badge = 0;

        str_list.push(line);
        if str_list.len() == 3 {
            common_badge = find_common_badge(&str_list[0], &str_list[1], &str_list[2]) as i32;
            str_list.clear();
            // if lowercase
            if common_badge > 96 {
                common_badge -= 96;
            } else {
                common_badge -= 38;
            }
        }

        // if lowercase
        if common_letter > 96 {
            common_letter -= 96;
        } else {
            common_letter -= 38;
        }

        total_score += common_letter;
        total_score_2 += common_badge;
    }
    println!("PART 1 | {}", total_score);
    println!("PART 2 | {}", total_score_2);
}
