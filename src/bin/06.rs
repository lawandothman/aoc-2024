use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let directions = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];

    let (mut x, mut y, mut dir_idx) = find_guard(&map).expect("No guard found");

    let mut visited = HashSet::new();
    visited.insert((x, y));

    loop {
        let (dx, dy) = directions[dir_idx];
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if nx < 0 || ny < 0 || ny >= map.len() as isize || nx >= map[0].len() as isize {
            break;
        }

        let (nx, ny) = (nx as usize, ny as usize);

        if map[ny][nx] == '#' {
            dir_idx = (dir_idx + 1) % 4;
        } else {
            x = nx;
            y = ny;
            visited.insert((x, y));
        }
    }

    Some(visited.len() as u32)
}

fn find_guard(map: &[Vec<char>]) -> Option<(usize, usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let dir_idx = match cell {
                '^' => Some(0), // up
                '>' => Some(1), // right
                'v' => Some(2), // down
                '<' => Some(3), // left
                _ => None,
            };
            if let Some(dir_idx) = dir_idx {
                return Some((x, y, dir_idx));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let directions = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];

    let (start_x, start_y, _) = find_guard(&map).expect("No guard found");
    let mut valid_pos_count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' || (x == start_x && y == start_y) {
                continue;
            }

            let mut map = map.clone();
            map[y][x] = '#';

            if is_stuck(&map, &directions) {
                valid_pos_count += 1;
            }
        }
    }
    Some(valid_pos_count)
}

fn is_stuck(map: &[Vec<char>], directions: &[(isize, isize)]) -> bool {
    let (mut x, mut y, mut dir_idx) = find_guard(map).expect("No guard found");
    let mut visited = HashSet::new();

    while visited.insert((x, y, dir_idx)) {
        let (dx, dy) = directions[dir_idx];
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if nx < 0 || ny < 0 || ny >= map.len() as isize || nx >= map[0].len() as isize {
            return false;
        }

        let (nx, ny) = (nx as usize, ny as usize);

        if map[ny][nx] == '#' {
            dir_idx = (dir_idx + 1) % 4;
        } else {
            x = nx;
            y = ny;
        }
    }
    true
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
