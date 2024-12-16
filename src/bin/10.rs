use advent_of_code::Direction;
use std::collections::HashSet;

advent_of_code::solution!(10);

struct Map {
    matrix: Vec<Vec<u32>>,
    trailheads: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}
impl Map {
    fn new(input: &str) -> Self {
        let matrix: Vec<Vec<u32>> = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();

        let trailheads: Vec<(usize, usize)> = matrix
            .iter()
            .enumerate()
            .flat_map(|(row_id, row)| {
                row.iter().enumerate().filter_map(move |(col_idx, &col)| {
                    if col == 0 {
                        Some((row_id, col_idx))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let width = matrix.len();
        let height = matrix[0].len();

        Self {
            matrix,
            trailheads,
            width,
            height,
        }
    }

    fn is_valid(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        Direction::iterator()
            .filter_map(|d| {
                let (nx, ny) = d.apply(x as isize, y as isize);
                if self.is_valid(nx, ny) {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    fn solve(&self, part2: bool) -> u32 {
        let mut res = 0;
        let mut res2 = 0;

        for (x, y) in self.trailheads.clone() {
            let mut stack = vec![(x, y)];
            let mut seen = HashSet::new();

            while let Some((cur_x, cur_y)) = stack.pop() {
                let cur_val = self.matrix[cur_x][cur_y];
                if cur_val == 9 {
                    seen.insert((cur_x, cur_y));
                    res2 += 1;
                    continue;
                }

                for (nx, ny) in self.neighbours(cur_x, cur_y) {
                    if self.matrix[nx][ny] == cur_val + 1 {
                        stack.push((nx, ny));
                    }
                }
            }
            res += seen.len() as u32;
        }

        if part2 {
            res2
        } else {
            res
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let res = map.solve(false);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let res = map.solve(true);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
