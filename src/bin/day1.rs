mod utils;

use utils::file::read_content;

fn part_one(content: &str) -> usize {
    let mut first_col: Vec<usize> = Vec::new();
    let mut second_col: Vec<usize> = Vec::new();
    let mut result = 0;

    for line in content.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            first_col.push(first.parse().unwrap());
            second_col.push(second.parse().unwrap());
        }

        first_col.sort();
        second_col.sort();
    }

    for i in 0..first_col.len() {
        let diff = if first_col[i] > second_col[i] {
            first_col[i] - second_col[i]
        } else {
            second_col[i] - first_col[i]
        };
        result += diff;
    }
    result
}

fn main() {
    let content = read_content("./src/input/day1.txt");
    let first = part_one(&content);
    println!("{first}");
}
