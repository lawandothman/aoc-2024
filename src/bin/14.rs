advent_of_code::solution!(14);

pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub struct Robot {
    pub position: Point,
    pub velocity: Point,
}

pub fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut p = parts[0][2..]
                .trim_end_matches(',')
                .split(',')
                .map(|n| n.parse::<isize>().unwrap());
            let mut v = parts[1][2..]
                .split(',')
                .map(|n| n.parse::<isize>().unwrap());

            Robot {
                position: Point::new(p.next().unwrap(), p.next().unwrap()),
                velocity: Point::new(v.next().unwrap(), v.next().unwrap()),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;
    const SECONDS: usize = 100;

    let robots = parse(input);
    let mut robots_data: Vec<((usize, usize), (isize, isize))> = robots
        .iter()
        .map(|r| {
            (
                (r.position.x as usize, r.position.y as usize),
                (r.velocity.x, r.velocity.y),
            )
        })
        .collect();

    for _ in 0..SECONDS {
        robots_data = robots_data
            .iter()
            .map(|&((x, y), (vx, vy))| {
                let new_x = ((x as isize + vx).rem_euclid(WIDTH as isize)) as usize;
                let new_y = ((y as isize + vy).rem_euclid(HEIGHT as isize)) as usize;
                ((new_x, new_y), (vx, vy))
            })
            .collect();
    }

    let mut tiles = vec![vec![0; WIDTH]; HEIGHT];
    for &((x, y), _) in &robots_data {
        tiles[y][x] += 1;
    }

    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    let mut quadrants = [0, 0, 0, 0];
    for (y, _) in tiles.iter().enumerate().take(HEIGHT) {
        for x in 0..WIDTH {
            if x == mid_x || y == mid_y {
                continue;
            }

            if x < mid_x && y < mid_y {
                quadrants[0] += tiles[y][x];
            } else if x > mid_x && y < mid_y {
                quadrants[1] += tiles[y][x];
            } else if x < mid_x && y > mid_y {
                quadrants[2] += tiles[y][x];
            } else if x > mid_x && y > mid_y {
                quadrants[3] += tiles[y][x];
            }
        }
    }

    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse(input);
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    const INV_W: i64 = 51;

    let bx = find_best_offset(&robots, WIDTH as u32, true) as i64;
    let by = find_best_offset(&robots, HEIGHT as u32, false) as i64;

    let t = bx + INV_W * (by - bx) * WIDTH;
    Some(t.rem_euclid(WIDTH * HEIGHT) as u32)
}

fn calculate_variance(positions: &[(u32, u32)]) -> f64 {
    let mean = positions.iter().map(|&(x, _)| x as f64).sum::<f64>() / positions.len() as f64;
    let variance = positions
        .iter()
        .map(|&(x, _)| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / positions.len() as f64;
    variance
}

fn find_best_offset(robots: &[Robot], modulo: u32, use_x: bool) -> u32 {
    let mut best_variance = f64::MAX;
    let mut best_offset = 0;

    for offset in 0..modulo {
        let positions: Vec<_> = move_robots(robots, modulo, modulo, offset).collect();
        let variance = if use_x {
            calculate_variance(&positions)
        } else {
            let swapped: Vec<(u32, u32)> = positions.iter().map(|&(x, y)| (y, x)).collect();
            calculate_variance(&swapped)
        };
        if variance < best_variance {
            best_variance = variance;
            best_offset = offset;
        }
    }

    best_offset
}

pub fn move_robots(
    robots: &[Robot],
    width: u32,
    height: u32,
    steps: u32,
) -> impl Iterator<Item = (u32, u32)> + '_ {
    let steps_x = steps % width;
    let steps_y = steps % height;

    robots.iter().map(move |robot| {
        let new_x = (robot.position.x + steps_x as isize * robot.velocity.x)
            .rem_euclid(width as isize) as u32;
        let new_y = (robot.position.y + steps_y as isize * robot.velocity.y)
            .rem_euclid(height as isize) as u32;
        (new_x, new_y)
    })
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
        assert_eq!(result, Some(5253));
    }
}
