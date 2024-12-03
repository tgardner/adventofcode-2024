advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mul = Vec::<(i32, i32)>::new();

    for captures in re.captures_iter(input) {
        let first_num = captures.get(1).unwrap().as_str().parse().unwrap();
        let second_num = captures.get(2).unwrap().as_str().parse().unwrap();
        mul.push((first_num, second_num));
    }

    let result = mul.iter().map(|f| f.0 * f.1).sum::<i32>();

    return Some(result as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
