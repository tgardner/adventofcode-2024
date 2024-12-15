advent_of_code::solution!(13);

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut lines = input.lines().map(|line| line.split_once(", ").unwrap());
        let (ax, ay) = lines.next().unwrap();
        let (ax, ay) = (
            ax.trim_start_matches("Button A: X+").parse().unwrap(),
            ay.trim_start_matches("Y+").parse().unwrap(),
        );

        let (bx, by) = lines.next().unwrap();
        let (bx, by) = (
            bx.trim_start_matches("Button B: X+").parse().unwrap(),
            by.trim_start_matches("Y+").parse().unwrap(),
        );

        let (prize_x, prize_y) = lines.next().unwrap();
        let (px, py) = (
            prize_x.trim_start_matches("Prize: X=").parse().unwrap(),
            prize_y.trim_start_matches("Y=").parse().unwrap(),
        );

        Self {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        }
    }

    fn solve(&self) -> Option<i64> {
        let d = self.ax * self.by - self.ay * self.bx;
        let di = self.px * self.by - self.py * self.bx;
        let dj = self.py * self.ax - self.px * self.ay;

        if di % d == 0 && dj % d == 0 {
            Some(3 * di / d + dj / d)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.split("\n\n").map(Machine::new).collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_input(input);
    let result = machines.iter().filter_map(|m| m.solve()).sum::<i64>();
    Some(result as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse_input(input);
    let result = machines
        .into_iter()
        .map(|mut m| {
            m.px += 10000000000000;
            m.py += 10000000000000;
            m
        })
        .filter_map(|m| m.solve())
        .sum::<i64>();
    Some(result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
