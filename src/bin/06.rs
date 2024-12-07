advent_of_code::solution!(6);

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone)]
struct Lab {
    map: Vec<Vec<char>>,
    guard_position: (usize, usize, usize), // x, y, direction
    width: usize,
    height: usize,
}

impl Lab {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let (width, height) = (map[0].len(), map.len());
        let mut guard_position = (0, 0, 0);

        // Find the initial position and direction of the guard
        for (row, line) in map.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if let Some(d) = "^>v<".find(cell) {
                    guard_position = (col, row, d);
                    break;
                }
            }
        }

        Lab {
            map,
            guard_position,
            width,
            height,
        }
    }

    fn visited(&self) -> usize {
        self.map.iter().flatten().filter(|&&c| c == 'X').count()
    }

    fn patrol(&mut self) -> bool {
        let (mut x, mut y, mut direction) = self.guard_position;
        self.map[y][x] = 'X';

        let mut obstacles = Vec::new();

        loop {
            let (dx, dy) = DIRECTIONS[direction];
            let nx = x as isize + dx as isize;
            let ny = y as isize + dy as isize;

            if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                return false;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if self.map[ny][nx] == '#' {
                if obstacles.contains(&(nx, ny, direction)) {
                    return true;
                }
                obstacles.push((nx, ny, direction));

                direction = (direction + 1) % 4;
            } else {
                x = nx;
                y = ny;
                self.map[y][x] = 'X';
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab = Lab::new(input);
    lab.patrol();
    Some(lab.visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lab = Lab::new(input);
    lab.patrol();

    let valid_obstructions = (0..lab.width)
        .flat_map(|x| (0..lab.height).map(move |y| (x, y)))
        .filter(|&(x, y)| lab.map[y][x] == 'X')
        .filter(|&(x, y)| {
            let mut temp = lab.clone();
            temp.map[y][x] = '#';
            temp.patrol()
        })
        .count();

    Some(valid_obstructions as u32)
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
