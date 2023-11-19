//https://adventofcode.com/2022/day/8
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool,
}

impl Tree {
    fn new(h: u8) -> Tree {
        Tree {
            height: h,
            visible: false,
        }
    }
}

fn main() {
    //open file handle
    let file = File::open("test.csv").expect("file not found");

    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    //store as vec of vecs
    let mut trees: Vec<Vec<Tree>> = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("Unable to read line");

        let mut tree_row: Vec<Tree> = Vec::new();

        for tree in line.chars() {
            tree_row.push(Tree::new(tree.to_digit(10).unwrap() as u8));
        }

        trees.push(tree_row);
    }

    //iterate left to right, then .rev() for right to left
    for mut row in &mut trees {
        //get left to right
        set_visible_trees(&mut row);

        println!("{:?}", row);

        //get right to left
        row.reverse();
        set_visible_trees(&mut row);
        row.reverse();
        println!("{:?}", row);
        println!("\n");
        //let tmp_row: Vec<u8> = ro/w.rev();
    }

    //iterate top to bottom (store in temp vec), then .rev() for bottom to top
    let total_cols: usize = *(&trees[0].len());
    for col_num in 0..total_cols {
        let mut tmp_row: Vec<Tree> = Vec::new();
        for mut row in &mut trees {
            tmp_row.push(row[col_num]);
        }

        println!("{:?}", tmp_row);
    }
}

fn set_visible_trees(row: &mut Vec<Tree>) {
    let mut largest_tree: u8 = 0;

    for mut tree in row {
        if tree.height > largest_tree {
            tree.visible = true;
            largest_tree = tree.height;
        }
    }
}

fn get_visible_tress(trees: Vec<Vec<Tree>>) -> u32 {
    return 0;
}
