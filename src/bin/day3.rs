mod utils;

use regex::Regex;
use utils::file::read_content;

fn part_one(content: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    content.lines().into_iter().for_each(|x| {
        for m in pattern.captures_iter(x) {
            result += &m[1].parse::<i32>().unwrap() * &m[2].parse::<i32>().unwrap()
        }
    });
    result
}

fn part_two(content: &str) -> usize {
    0
}

fn main() {
    let content = read_content("./src/input/day3.txt");
    let first = part_one(&content);
    println!("Part one: {first}");

    let second = part_two(&content);
    println!("Part two: {second}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_STR), 161)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_STR), 0)
    }
}
