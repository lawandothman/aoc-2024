use std::ops::{Add, AddAssign};

advent_of_code::solution!(15);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn up() -> Self {
        Self::new(0, -1)
    }

    pub fn down() -> Self {
        Self::new(0, 1)
    }

    pub fn left() -> Self {
        Self::new(-1, 0)
    }

    pub fn right() -> Self {
        Self::new(1, 0)
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Self::up(),
            b'v' => Self::down(),
            b'<' => Self::left(),
            b'>' => Self::right(),
            _ => unreachable!(),
        }
    }
}

impl From<Point> for u8 {
    fn from(value: Point) -> Self {
        match value {
            Point { x: 0, y: -1 } => b'^',
            Point { x: 0, y: 1 } => b'v',
            Point { x: -1, y: 0 } => b'<',
            Point { x: 1, y: 0 } => b'>',
            _ => unreachable!(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

fn parse(input: &str, part: u8) -> (Vec<Vec<u8>>, Vec<&u8>, Point) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut robot = None;

    (
        grid.lines()
            .enumerate()
            .map(|(y, line)| {
                if part == 1 {
                    line.bytes()
                        .enumerate()
                        .map(|(x, b)| {
                            if b == b'@' {
                                robot = Some(Point::new(x as i32, y as i32));
                                b'.'
                            } else {
                                b
                            }
                        })
                        .collect()
                } else {
                    line.bytes()
                        .enumerate()
                        .flat_map(|(x, b)| match b {
                            b'#' => [b'#', b'#'],
                            b'O' => [b'[', b']'],
                            b'.' => [b'.', b'.'],
                            b'@' => {
                                robot = Some(Point::new(x as i32 * 2, y as i32));
                                [b'.', b'.']
                            }
                            _ => unreachable!(),
                        })
                        .collect()
                }
            })
            .collect(),
        moves.lines().flat_map(|line| line.as_bytes()).collect(),
        robot.unwrap(),
    )
}

fn coordinates(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'O' || *b == b'[' {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut grid, moves, mut robot) = parse(input, 1);

    for &m in moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                robot = next;
            }
            b'O' => {
                let mut boxes = vec![next];
                let mut path = next + direction;

                while grid[path.y as usize][path.x as usize] == b'O' {
                    boxes.push(path);
                    path += direction;
                }

                if grid[path.y as usize][path.x as usize] == b'.' {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = b'O';
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    Some(coordinates(&grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut grid, moves, mut robot) = parse(input, 2);

    for &m in moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                robot = next;
            }
            side @ b'[' | side @ b']' => {
                let mut boxes = vec![next];

                if side == b'[' {
                    boxes.push(next + Point::right());
                } else {
                    boxes.push(next + Point::left());
                }

                let mut blocked = false;

                match m {
                    b'^' | b'v' => {
                        let mut current = boxes.clone();

                        while current.len() > 1 {
                            let mut next = Vec::new();

                            for b in current {
                                let path = b + direction;

                                match grid[path.y as usize][path.x as usize] {
                                    b'#' => {
                                        blocked = true;
                                        next.clear();
                                        break;
                                    }
                                    side @ b'[' | side @ b']' => {
                                        if !next.contains(&path) {
                                            boxes.push(path);
                                            next.push(path);

                                            if side == b'[' {
                                                boxes.push(path + Point::right());
                                                next.push(path + Point::right());
                                            } else {
                                                boxes.push(path + Point::left());
                                                next.push(path + Point::left());
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            current = next;
                        }
                    }
                    b'<' | b'>' => {
                        let mut path = next + direction + direction;

                        while [b'[', b']'].contains(&grid[path.y as usize][path.x as usize]) {
                            boxes.push(path);
                            path += direction;
                        }

                        if grid[path.y as usize][path.x as usize] != b'.' {
                            blocked = true;
                        }
                    }
                    _ => {}
                }

                if !blocked {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = grid[b.y as usize][b.x as usize];
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    Some(coordinates(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
