advent_of_code::solution!(2);

const ASCENDING: i32 = 1;
const DESCENDING: i32 = -1;

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines();
    lines
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .filter(|v: &Vec<i32>| !v.is_empty())
        .collect()
}

fn is_ordered(vec: &Vec<i32>) -> bool {
    return is_ordered_signum(vec, ASCENDING) || is_ordered_signum(vec, DESCENDING);
}

fn is_ordered_signum(vec: &Vec<i32>, signum: i32) -> bool {
    vec.windows(2)
        .all(|w| advent_of_code::signum(w[0] - w[1]) == signum && w[0].abs_diff(w[1]) <= 3)
}

fn is_one_off(vec: &Vec<i32>) -> bool {
    for i in 0..vec.len() {
        let mut temp = vec.clone();
        temp.remove(i);
        if is_ordered(&temp) {
            return true;
        }
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    let result = reports.iter().filter(|r| is_ordered(r)).count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    let result = reports
        .iter()
        .filter(|r| is_ordered(r) || is_one_off(r))
        .count();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
