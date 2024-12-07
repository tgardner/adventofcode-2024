use std::collections::HashSet;

advent_of_code::solution!(6);

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Lab {
    map: Vec<Vec<char>>,
    guard_position: (usize, usize),
    direction: usize,
}

impl Lab {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut guard_position = (0, 0); // Guard's position
        let mut direction = 0;

        // Find the initial position and direction of the guard
        for (row, line) in map.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if let Some(d) = "^>v<".find(cell) {
                    guard_position = (col, row);
                    direction = d;
                    break;
                }
            }
        }

        Lab {
            map,
            guard_position,
            direction,
        }
    }

    fn height(&self) -> usize {
        self.map.len()
    }
    fn width(&self) -> usize {
        self.map[0].len()
    }
}

fn simulate(lab: &Lab) -> (HashSet<(usize, usize)>, bool) {
    let height = lab.height();
    let width = lab.width();
    let mut cycle = false;

    let (mut x, mut y) = lab.guard_position;
    let mut direction = lab.direction;

    let mut obstacles = Vec::new();
    let mut visited = HashSet::new();
    visited.insert((x, y));

    loop {
        let (dx, dy) = DIRECTIONS[direction];
        let nx = x as isize + dx as isize;
        let ny = y as isize + dy as isize;

        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            break; // Guard leaves the map
        }

        let (nx, ny) = (nx as usize, ny as usize);

        if lab.map[ny][nx] == '#' {
            if obstacles.contains(&(nx, ny, direction)) {
                cycle = true;
                break; // Guard has visited this position before
            }
            obstacles.push((nx, ny, direction));

            direction = (direction + 1) % 4;
        } else {
            x = nx;
            y = ny;
            visited.insert((x, y));
        }
    }

    (visited, cycle)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lab = Lab::new(input);
    let (visited, _) = simulate(&lab);

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lab = Lab::new(input);
    let mut valid_obstructions = 0;

    let (visited, _) = simulate(&lab);
    for (x, y) in &visited {
        if (*x, *y) == lab.guard_position {
            continue;
        }

        lab.map[*y][*x] = '#';

        let (_, cycle) = simulate(&lab);
        if cycle {
            valid_obstructions += 1;
        }

        lab.map[*y][*x] = '.';
    }

    Some(valid_obstructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
