advent_of_code::solution!(19);

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns: Vec<&str> = lines
        .next()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim())
        .collect();

    lines.next(); // Skip the blank line

    let designs: Vec<&str> = lines.collect();

    (patterns, designs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, designs) = parse(input);

    let mut possible_designs = 0;
    for design in designs {
        if count_ways_dp(design, &patterns) > 0 {
            possible_designs += 1;
        }
    }

    Some(possible_designs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (patterns, designs) = parse(input);

    let mut total_ways = 0;
    for design in designs {
        total_ways += count_ways_dp(design, &patterns);
    }

    Some(total_ways)
}

fn count_ways_dp(design: &str, patterns: &[&str]) -> usize {
    let n = design.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1; // 1 way to make an empty design

    for i in 0..n {
        if dp[i] > 0 {
            for pattern in patterns {
                if i + pattern.len() <= n && design[i..].starts_with(pattern) {
                    dp[i + pattern.len()] += dp[i];
                }
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
