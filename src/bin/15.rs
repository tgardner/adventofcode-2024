use advent_of_code::Direction;

advent_of_code::solution!(15);

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<char>>,
    moves: Vec<Direction>,
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
        let moves: Vec<Direction> = input
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| Direction::try_from(c).ok())
            .collect();
        let robot = find_robot(&map);

        Self { map, moves, robot }
    }
}

impl Warehouse {
    fn from_wide(input: &str) -> Self {
        let mut warehouse = Warehouse::from(input);

        warehouse.map = warehouse
            .map
            .clone()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        '#' => vec!['#', '#'],
                        '.' => vec!['.', '.'],
                        'O' => vec!['[', ']'],
                        '@' => vec!['@', '.'],
                        _ => vec![],
                    })
                    .flatten()
                    .collect()
            })
            .collect();
        warehouse.robot = find_robot(&warehouse.map);

        warehouse
    }

    #[allow(dead_code)]
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

            if self.can_move(x, y, *d) {
                self.do_move(x, y, *d);
            }
        }
    }

    fn can_move(&self, x: usize, y: usize, direction: Direction) -> bool {
        let (nx, ny) = direction.apply(x as isize, y as isize);
        let (nx, ny) = (nx as usize, ny as usize);

        match self.map[ny][nx] {
            '#' => false,
            '.' => true,
            'O' => self.can_move(nx, ny, direction),
            '[' => {
                self.can_move(nx, ny, direction)
                    && (y == ny || self.can_move(nx + 1, ny, direction))
            }
            ']' => {
                self.can_move(nx, ny, direction)
                    && (y == ny || self.can_move(nx - 1, ny, direction))
            }
            _ => false,
        }
    }

    fn do_move(&mut self, x: usize, y: usize, direction: Direction) {
        let (nx, ny) = direction.apply(x as isize, y as isize);
        let (nx, ny) = (nx as usize, ny as usize);
        let cell = self.map[ny][nx];

        match cell {
            '#' => panic!("Trying to move a wall"),
            'O' => self.do_move(nx, ny, direction),
            '[' => {
                self.do_move(nx, ny, direction);
                if y != ny {
                    self.do_move(nx + 1, ny, direction);
                }
            }
            ']' => {
                self.do_move(nx, ny, direction);
                if y != ny {
                    self.do_move(nx - 1, ny, direction);
                }
            }
            _ => (),
        }

        self.map[ny][nx] = self.map[y][x];
        self.map[y][x] = '.';
        if self.map[ny][nx] == '@' {
            self.robot = (nx, ny);
        }
    }

    fn gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().map(
                    move |(c, cell)| {
                        if "O[".contains(*cell) {
                            100 * r + c
                        } else {
                            0
                        }
                    },
                )
            })
            .sum()
    }
}

fn find_robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut robot = (0, 0);
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == '@' {
                robot = (col, row);
                break;
            }
        }
    }
    robot
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::from(input);
    warehouse.patrol();
    Some(warehouse.gps())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::from_wide(input);
    warehouse.patrol();
    Some(warehouse.gps())
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
