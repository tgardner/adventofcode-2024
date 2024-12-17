advent_of_code::solution!(17);

struct Computer<'a> {
    program: &'a [u64],
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
}

impl Computer<'_> {
    fn run(&mut self) -> Option<u64> {
        while self.ip < self.program.len() {
            let combo = |index| match self.program[index] {
                0..4 => self.program[index],
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            };

            match self.program[self.ip] {
                0 => self.a >>= combo(self.ip + 1),
                1 => self.b ^= self.program[self.ip + 1],
                2 => self.b = combo(self.ip + 1) % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = self.program[self.ip + 1] as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    let out = combo(self.ip + 1) % 8;
                    self.ip += 2;
                    return Some(out);
                }
                6 => self.b = self.a >> combo(self.ip + 1),
                7 => self.c = self.a >> combo(self.ip + 1),
                _ => unreachable!(),
            }

            self.ip += 2;
        }

        None
    }
}

pub fn parse(input: &str) -> Vec<u64> {
    input.split(|c: char| !c.is_digit(10)) // Split on non-digits
        .filter(|s| !s.is_empty()) // Remove empty strings
        .filter_map(|s| s.parse::<u64>().ok()) // Parse and filter out errors
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let input = parse(input);
    let mut computer =
        Computer { program: &input[3..], ip: 0, a: input[0], b: input[1], c: input[2] };
    let mut out = Vec::new();

    while let Some(n) = computer.run() {
        let digit = (n as u8 + b'0') as char;
        out.push(digit);
        out.push(',');
    }

    out.pop();
    let result = out.iter().collect();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut valid = vec![0];
    let input = parse(input);

    for &out in input.iter().skip(3).rev() {
        let mut next = Vec::new();

        for v in valid {
            for n in 0..8 {
                let a = (v << 3) | n;
                let mut computer =
                    Computer { program: &input[3..], ip: 0, a, b: input[1], c: input[2] };

                if let Some(result) = computer.run() {
                    if result == out {
                        next.push(a);
                    }
                }
            }
        }

        valid = next;
    }

    valid.iter().min().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&binding);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }
    #[test]
    fn test_part_two() {
        let binding = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(&binding);
        assert_eq!(result, Some(117440));
    }
}
