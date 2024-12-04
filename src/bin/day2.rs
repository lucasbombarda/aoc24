mod utils;

use utils::file::read_content;

#[derive(Debug, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn check(nums: &Vec<isize>) -> isize {
    let mut is_valid = true;
    let mut direction: Option<Direction> = None;

    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];

        if diff.abs() > 3 {
            is_valid = false;
        }

        if let Some(dir) = &direction {
            match dir {
                Direction::Ascending if diff <= 0 => {
                    is_valid = false;
                }
                Direction::Descending if diff >= 0 => {
                    is_valid = false;
                }
                _ => {}
            }
        } else {
            direction = if diff > 0 {
                Some(Direction::Ascending)
            } else if diff < 0 {
                Some(Direction::Descending)
            } else {
                is_valid = false;
                None
            }
        }
    }
    if is_valid {
        1
    } else {
        0
    }
}

fn check_two(nums: &Vec<isize>) -> isize {
    if check(nums) == 1 {
        return 1;
    }

    let max_idx = nums.len() - 1;
    for i in 0..=max_idx {
        let mut nums_wo_idx: Vec<isize> = Vec::new();
        for (idx, el) in nums.iter().enumerate() {
            if idx != i {
                nums_wo_idx.push(*el);
            }
        }

        if check(&nums_wo_idx) == 1 {
            return 1;
        }
    }
    return 0;
}

fn part_one(content: &str) -> isize {
    let mut result = 0;
    for line in content.lines() {
        let records = line
            .split_whitespace()
            .map(|x| (*x).parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        result += check(&records);
    }
    result
}

fn part_two(content: &str) -> isize {
    let mut result = 0;
    for line in content.lines() {
        let records = line
            .split_whitespace()
            .map(|x| (*x).parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        result += check_two(&records);
    }
    result
}

fn main() {
    let content = read_content("./src/input/day2.txt");
    let first = part_one(&content);
    println!("Part one: {first}");

    let second = part_two(&content);
    println!("Part two: {second}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4);
    }
}
