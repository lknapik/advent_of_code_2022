//https://adventofcode.com/2022/day/2
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");

    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);   

    let mut total_score: i32 = 0;

    for line in buf_reader.lines() {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");
        //Get each cast, convert to rock, paper, scissors as 0, 1, 2.
        let elf_cast = line.chars().nth(0).expect("Something") as i32 - 65;
        let my_cast = line.chars().nth(2).expect("Something") as i32 - 88;

        //Subtract mine vs elf and modulus 3
        let winner = (my_cast - elf_cast).rem_euclid(3);

        //Trust this math works
        total_score += (3*winner + 3).rem_euclid(9);

        //Add one point to my cast for selection score
        total_score+= my_cast+1;

    }
    println!("PART 1 | Your total score is: {}", total_score);


    /*PART 2*/
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");

    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);   


    total_score = 0;
    for line in buf_reader.lines(){
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");
        let elf_cast = line.chars().nth(0).expect("Something") as i32 - 65;
        //0 is loss, 1 is draw, 2 is win
        //2 was loss 0 was draw 1 was win
        let winner = (line.chars().nth(2).expect("Something") as i32 - 89).rem_euclid(3);

        let my_cast = (elf_cast + winner).rem_euclid(3);

        total_score += (3*winner + 3).rem_euclid(9);
        total_score += my_cast+1;
        
    }
    println!("PART 2 | Your total score is: {}", total_score);

}

