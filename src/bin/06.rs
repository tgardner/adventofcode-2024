use std::collections::HashSet;

advent_of_code::solution!(6);

struct Lab {
    map: Vec<Vec<char>>,
    guard_position: (usize, usize),
    direction: (isize, isize),
}

impl Lab {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut guard_position = (0, 0); // Guard's position
        let mut direction = (0, -1); // Guard's direction (dx, dy)

        // Find the initial position and direction of the guard
        for (row, line) in map.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if "^v<>".contains(cell) {
                    guard_position = (col, row);
                    direction = match cell {
                        '^' => (0, -1),
                        'v' => (0, 1),
                        '<' => (-1, 0),
                        '>' => (1, 0),
                        _ => panic!("Invalid guard direction!"),
                    };
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

fn simulate(lab: &Lab) -> (usize, bool) {
    let height = lab.height();
    let width = lab.width();
    let mut cycle = false;

    let (mut x, mut y) = lab.guard_position;
    let mut direction = lab.direction;

    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(lab.guard_position);

    loop {
        let (dx, dy) = direction;
        let nx = x as isize + dx as isize;
        let ny = y as isize + dy as isize;

        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            break; // Guard leaves the map
        }

        let (nx, ny) = (nx as usize, ny as usize);

        if lab.map[ny][nx] == '#' {
            if obstacles.contains(&(nx, ny, dx, dy)) {
                cycle = true;
                break; // Guard has visited this position before
            }
            obstacles.insert((nx, ny, dx, dy));

            // Turn right 90 degrees
            direction = match direction {
                (0, -1) => (1, 0),  // Up -> Right
                (1, 0) => (0, 1),   // Right -> Down
                (0, 1) => (-1, 0),  // Down -> Left
                (-1, 0) => (0, -1), // Left -> Up
                _ => panic!("Invalid direction!"),
            };
        } else {
            // Move forward
            x = nx;
            y = ny;
            visited.insert((x, y));
        }
    }

    (visited.len(), cycle)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lab = Lab::new(input);
    let (visited, _) = simulate(&lab);

    Some(visited as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lab = Lab::new(input);
    let mut valid_obstructions = 0;

    for y in 0..lab.height() {
        for x in 0..lab.width() {
            if lab.map[y][x] == '.' && (x, y) != lab.guard_position {
                lab.map[y][x] = '#';

                let (_, cycle) = simulate(&lab);
                if cycle {
                    valid_obstructions += 1;
                }

                lab.map[y][x] = '.';
            }
        }
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
