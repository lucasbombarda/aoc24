mod utils;

use std::{collections::HashMap, usize};
use utils::file::read_content;

fn part_one(content: &str) -> i32 {
    let mut map_first_col: HashMap<usize, usize> = HashMap::new();
    let mut map_second_col: HashMap<usize, usize> = HashMap::new();

    for line in content.lines() {
        if let Some((num_first_col, num_second_col)) = line.split_once("   ") {
            let first_num = num_first_col.parse::<usize>().unwrap();
            let sec_num = num_second_col.parse::<usize>().unwrap();

            map_first_col
                .entry(first_num)
                .and_modify(|x| *x += 1)
                .or_insert(1);

            map_second_col
                .entry(sec_num)
                .and_modify(|x| *x += 1)
                .or_insert(1);

            println!("{num_first_col} - {num_second_col}");
        }
    }

    println!("{:?}", map_first_col);
    println!("{:?}", map_second_col);
    1
}

fn main() {
    let content = read_content("./src/input/day1.txt");
    let first = part_one(&content);
    println!("{first}");
}
