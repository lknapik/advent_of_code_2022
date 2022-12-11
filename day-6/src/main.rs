//https://adventofcode.com/2022/day/6
use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;


fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}


fn main() {
    //open file handle
    let mut file = File::open("real.txt")
        .expect("file not found");

    let mut data = String::new();
    
    file.read_to_string(&mut data).expect("data");

    let mut data_iter = data.chars();
    let mut buffer = VecDeque::new();
    let mut count = 0;
    //initial populate

    for _ in 0..14 {
        buffer.push_back(data_iter.next().unwrap());
        count += 1;
    }

    while !has_unique_elements(&buffer){
        buffer.pop_front();
        buffer.push_back(data_iter.next().unwrap());
        count += 1;
    }
    println!("First unique at: {}", count);
}
