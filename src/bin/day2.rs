mod utils;

use utils::file::read_content;

fn part_one(content: &str) -> usize {
    0
}

fn part_two(content: &str) -> usize {
    0
}

fn main() {
    let content = read_content("./src/input/day2.txt");
    let first = part_one(&content);
    println!("Part one: {first}");

    let second = part_two(&content);
    println!("Part two: {second}");
}
