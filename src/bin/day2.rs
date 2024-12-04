mod utils;

use utils::file::read_content;

#[derive(Debug, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn part_one(content: &str) -> usize {
    let mut result = 0;
    for line in content.lines() {
        let records = line
            .split_whitespace()
            .map(|x| (*x).parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let mut direction: Option<Direction> = None;
        let mut is_valid = true;

        for i in 0..records.len() - 1 {
            let diff = records[i + 1] - records[i];

            if diff.abs() > 3 {
                is_valid = false;
                break;
            }

            if let Some(dir) = &direction {
                match dir {
                    Direction::Ascending if diff <= 0 => {
                        is_valid = false;
                        break;
                    }
                    Direction::Descending if diff >= 0 => {
                        is_valid = false;
                        break;
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
                    break;
                }
            }
        }
        if is_valid {
            result += 1;
        }
    }
    result
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
