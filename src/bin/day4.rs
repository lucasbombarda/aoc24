mod utils;

use utils::file::read_content;

fn part_one(content: &str) -> usize {
    0
}

fn part_two(content: &str) -> usize {
    0
}

fn main() {
    let content = read_content("./src/input/day4.txt");
    let first = part_one(&content);
    println!("Part one: {first}");

    let second = part_two(&content);
    println!("Part two: {second}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_ONE: &str = r#"....XXMAS.
    .SAMXMS...
    ...S..A...
    ..A.A.MS.X
    XMASAMX.MM
    X.....XA.A
    S.S.S.S.SS
    .A.A.A.A.A
    ..M.M.M.MM
    .X.X.XMASX
    "#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_PART_ONE), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_PART_ONE), 0);
    }
}
