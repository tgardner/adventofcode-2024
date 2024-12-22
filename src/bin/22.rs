advent_of_code::solution!(22);

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn next(mut n: usize) -> usize {
    n ^= n << 6;
    n &= 0xffffff;

    n ^= n >> 5;
    n &= 0xffffff;

    n ^= n << 11;
    n &= 0xffffff;

    n
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut result = 0;

    for number in input {
        let mut number = number;

        for _ in 0..2000 {
            number = next(number);
        }

        result += number;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut result = vec![0; 130321];
    let mut seen = vec![usize::MAX; 130321];

    let to_index = |previous: usize, current: usize| 9 + current % 10 - previous % 10;

    for (id, number) in input.iter().enumerate() {
        let zeroth = *number;
        let first = next(zeroth);
        let second = next(first);
        let third = next(second);

        let mut a;
        let mut b = to_index(zeroth, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);

        let mut number = third;

        for _ in 3..2000 {
            let previous = number;
            number = next(number);

            (a, b, c, d) = (b, c, d, to_index(previous, number));

            let key = 6859 * a + 361 * b + 19 * c + d;

            if seen[key] != id {
                result[key] += number % 10;
                seen[key] = id;
            }
        }
    }

    let result = *result.iter().max().unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
