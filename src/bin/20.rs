use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;
use advent_of_code::util::thread::*;

advent_of_code::solution!(20);

fn parse(input: &str) -> Grid<i32> {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    let mut time = grid.same_size_with(i32::MAX);
    let mut elapsed = 0;

    let mut position = start;
    let mut direction = ORTHOGONAL.into_iter().find(|&o| grid[position + o] != b'#').unwrap();

    while position != end {
        time[position] = elapsed;
        elapsed += 1;

        direction = [direction, direction.clockwise(), direction.counter_clockwise()]
            .into_iter()
            .find(|&d| grid[position + d] != b'#')
            .unwrap();
        position += direction;
    }

    time[end] = elapsed;
    time
}

fn count_cheats(time: &Grid<i32>, savings: u32) -> u32 {
    let mut cheats = 0;

    for y in 1..time.height - 1 {
        for x in 1..time.width - 1 {
            let point = Point::new(x, y);

            if time[point] != i32::MAX {
                cheats += check(&time, point, Point::new(2, 0), savings);
                cheats += check(&time, point, Point::new(0, 2), savings);
            }
        }
    }

    cheats
}

fn count_cheats_p2(time: &Grid<i32>, savings: u32) -> u32 {
    let mut items = Vec::with_capacity(10_000);

    for y in 1..time.height - 1 {
        for x in 1..time.width - 1 {
            let point = Point::new(x, y);

            if time[point] != i32::MAX {
                items.push(point);
            }
        }
    }

    let total = AtomicU32::new(0);
    spawn_batches(items, |batch| worker(&time, &total, batch, savings));
    total.into_inner()
}

pub fn part_one(input: &str) -> Option<u32> {
    let time = parse(input);
    let cheats = count_cheats(&time, 100);
    Some(cheats)
}

pub fn part_two(input: &str) -> Option<u32> {
    let time = parse(input);
    let res = count_cheats_p2(&time, 100);
    Some(res)
}

fn worker(time: &Grid<i32>, total: &AtomicU32, batch: Vec<Point>, savings: u32) {
    let mut cheats = 0;

    for point in batch {
        for x in 2..21 {
            cheats += check(time, point, Point::new(x, 0), savings);
        }

        for y in 1..21 {
            for x in (y - 20)..(21 - y) {
                cheats += check(time, point, Point::new(x, y), savings);
            }
        }
    }

    total.fetch_add(cheats, Ordering::Relaxed);
}

fn check(time: &Grid<i32>, first: Point, delta: Point, savings: u32) -> u32 {
    let second = first + delta;

    (time.contains(second)
        && time[second] != i32::MAX
        && (time[first] - time[second]).abs() - first.manhattan(second) >= savings as i32) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let time = parse(&input);
        let result = count_cheats(&time, 64);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let time = parse(&input);
        let result = count_cheats_p2(&time, 76);
        assert_eq!(result, 3);
    }
}
