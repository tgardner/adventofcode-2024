use itertools::Itertools;

advent_of_code::solution!(14);

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

impl From<&str> for Robot {
    fn from(line: &str) -> Self {
        let (pos, velocity) = line[2..].split_once(" v=").unwrap();
        let (x, y) = pos.split_once(',').unwrap();
        let (vx, vy) = velocity.split_once(',').unwrap();
        Self {
            p: (x.parse().unwrap(), y.parse().unwrap()),
            v: (vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}

impl Robot {
    fn predict(&mut self, secs: isize) {
        self.p.0 = (self.p.0 + secs * self.v.0).rem_euclid(WIDTH);
        self.p.1 = (self.p.1 + secs * self.v.1).rem_euclid(HEIGHT);
    }
    fn is_safe(&self) -> bool {
        self.p.0 != WIDTH / 2 && self.p.1 != HEIGHT / 2
    }
    fn quadrant(&self) -> usize {
        if self.p.0 < WIDTH / 2 {
            if self.p.1 < HEIGHT / 2 {
                return 0;
            }
            return 1;
        } else if self.p.1 < HEIGHT / 2 {
            return 2;
        }
        3
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut quadrants = vec![0; 4];
    let mut robots = parse_input(input);
    robots.iter_mut().for_each(|robot| {
        robot.predict(100);
        
        if robot.is_safe() {
            quadrants[robot.quadrant()] += 1;
        }
    });
    let result = quadrants.into_iter().product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut robots = parse_input(input);
    let result = (1..=101 * 103)
        .map(|t: usize| {
            (
                t,
                robots
                    .iter_mut()
                    .map(|r| {
                        r.predict(1);
                        r.quadrant()
                    })
                    .counts()
                    .values()
                    .product::<usize>(),
            )
        })
        .min_by(|&x, &y| x.1.cmp(&y.1))
        .unwrap()
        .0;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(69));
    }
}
