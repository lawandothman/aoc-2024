use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((x as i32, y as i32, c));
            }
        }
    }
    let height = input.lines().count() as i32;
    let width = input.lines().map(|l| l.chars().count()).max().unwrap_or(0) as i32;

    let mut freq_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (x, y, c) in antennas {
        freq_map.entry(c).or_default().push((x, y));
    }
    let mut antinodes = HashSet::new();

    for positions in freq_map.values() {
        let n = positions.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                antinodes.insert((2 * x1 - x2, 2 * y1 - y2));

                antinodes.insert((2 * x2 - x1, 2 * y2 - y1));

                let c_third_x = 2 * x1 + x2;
                let c_third_y = 2 * y1 + y2;
                if c_third_x % 3 == 0 && c_third_y % 3 == 0 {
                    antinodes.insert((c_third_x / 3, c_third_y / 3));
                }

                let c_twothirds_x = x1 + 2 * x2;
                let c_twothirds_y = y1 + 2 * y2;
                if c_twothirds_x % 3 == 0 && c_twothirds_y % 3 == 0 {
                    antinodes.insert((c_twothirds_x / 3, c_twothirds_y / 3));
                }
            }
        }
    }

    antinodes.retain(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height);

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((x as i32, y as i32, c));
            }
        }
    }

    let height = input.lines().count() as i32;
    let width = input.lines().map(|l| l.chars().count()).max().unwrap_or(0) as i32;

    let mut freq_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (x, y, c) in antennas {
        freq_map.entry(c).or_default().push((x, y));
    }

    let mut antinodes = HashSet::new();

    for positions in freq_map.values() {
        if positions.len() < 2 {
            continue;
        }
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 - x1;
                let dy = y2 - y1;
                let g = gcd(dx, dy);
                let step_x = dx / g;
                let step_y = dy / g;

                {
                    let (mut cx, mut cy) = (x1, y1);
                    while cx >= 0 && cy >= 0 && cx < width && cy < height {
                        antinodes.insert((cx, cy));
                        cx += step_x;
                        cy += step_y;
                    }
                }

                {
                    let (mut cx, mut cy) = (x1 - step_x, y1 - step_y);
                    while cx >= 0 && cy >= 0 && cx < width && cy < height {
                        antinodes.insert((cx, cy));
                        cx -= step_x;
                        cy -= step_y;
                    }
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
