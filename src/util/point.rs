use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
// Left to right and top to bottom.
pub const DIAGONAL: [Point; 8] = [
    Point::new(-1, -1),
    UP,
    Point::new(1, -1),
    LEFT,
    RIGHT,
    Point::new(-1, 1),
    DOWN,
    Point::new(1, 1),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    #[must_use]
    pub fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub fn signum(self, other: Self) -> Self {
        Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }
}

impl From<u8> for Point {
    #[inline]
    #[must_use]
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => UP,
            b'v' | b'D' => DOWN,
            b'<' | b'L' => LEFT,
            b'>' | b'R' => RIGHT,
            _ => unreachable!(),
        }
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
