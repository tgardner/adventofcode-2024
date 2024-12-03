advent_of_code::solution!(3);

use regex::Regex;

fn run_program(input: &str, cmd_pattern: &Regex) -> i32 {
    let mut running = true;
    let mut total = 0;

    for captures in cmd_pattern.captures_iter(input) {
        let cmd = captures.get(0).unwrap().as_str().split('(').next().unwrap();
        match cmd {
            "do" => running = true,
            "don't" => running = false,
            "mul" if running => {
                let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
                total += a * b;
            }
            _ => {}
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<i32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = run_program(input, &pattern);

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let result = run_program(input, &pattern);

    Some(result)
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
