advent_of_code::solution!(25);

const MASK: u64 = 0b011111_011111_011111_011111_011111_011111_011111;

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.bytes().map(|b| (b & 1) as u64);
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let mut result = 0;

    while let Some(first) = iter.next() {
        let bits = (0..40).fold(first, |bits, _| (bits << 1) | iter.next().unwrap());

        if bits & 1 == 0 {
            locks.push(bits & MASK);
        } else {
            keys.push(bits & MASK);
        }

        iter.next();
        iter.next();
    }

    for lock in &locks {
        for key in &keys {
            result += (lock & key == 0) as u32;
        }
    }

    Some(result)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
