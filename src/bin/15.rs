use advent_of_code::DIRECTIONS;

advent_of_code::solution!(15);

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<char>>,
    moves: Vec<usize>,
    robot: (usize, usize),
}

impl From<&str> for Warehouse {
    fn from(input: &str) -> Self {
        let mut input = input.trim().split("\n\n");
        let map: Vec<Vec<char>> = input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let moves: Vec<usize> = input
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| "^>v<".find(c))
            .collect();

        let mut robot = (0, 0);
        for (row, line) in map.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if cell == '@' {
                    robot = (col, row);
                    break;
                }
            }
        }

        Self { map, moves, robot }
    }
}

impl Warehouse {
    fn print(&self) {
        for row in self.map.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn patrol(&mut self) {
        for d in self.moves.clone().iter() {
            let (x, y) = self.robot;
            self.try_move(x, y, *d);
        }
    }

    fn try_move(&mut self, x: usize, y: usize, direction: usize) -> bool {
        let (dx, dy) = DIRECTIONS[direction];
        let (nx, ny) = (x as isize + dx as isize, y as isize + dy as isize);
        let (nx, ny) = (nx as usize, ny as usize);

        let m = match self.map[ny][nx] {
            '#' => false,
            '.' => true,
            'O' => self.try_move(nx, ny, direction),
            _ => false,
        };
        if m {
            self.map[ny][nx] = self.map[y][x];
            self.map[y][x] = '.';
            if self.map[ny][nx] == '@' {
                self.robot = (nx, ny);
            }
        }
        m
    }

    fn gps(&self) -> usize {
        let mut coord = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col] == 'O' {
                    coord += 100 * row + col;
                }
            }
        }
        coord
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::from(input);
    warehouse.patrol();
    Some(warehouse.gps())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_big() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
