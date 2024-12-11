mod utils;

use regex::Regex;
use utils::file::read_content;

fn part_one(content: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    content.lines().into_iter().for_each(|x| {
        pattern
            .captures_iter(x)
            .for_each(|m| result += &m[1].parse::<i32>().unwrap() * &m[2].parse::<i32>().unwrap());
    });
    result
}

fn part_two(content: &str) -> usize {
    let pattern_dont = Regex::new(r"don't\(\)").unwrap();
    let pattern_do = Regex::new(r"do\(\)").unwrap();
    let pattern_mult = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut doing = true;
    let mut result = 0;

    let commands = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

    for command in commands.find_iter(content) {
        let cmd = command.as_str();
        if pattern_dont.is_match(cmd) {
            doing = false;
        } else if pattern_do.is_match(cmd) {
            doing = true;
        } else if let Some(caps) = pattern_mult.captures(cmd) {
            if doing {
                let a = caps[1].parse::<usize>().unwrap();
                let b = caps[2].parse::<usize>().unwrap();
                result += a * b;
            }
        }
    }
    result
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

    const TEST_STR_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_STR_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_STR_1), 161)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_STR_2), 48)
    }
}
