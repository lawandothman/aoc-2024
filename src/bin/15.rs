advent_of_code::solution!(15);

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Floor,
    Box,
    Robot,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            '.' => Tile::Floor,
            _ => Tile::Floor,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn parse_input(input: &str) -> (Vec<&str>, String) {
    let lines: Vec<&str> = input.lines().collect();
    let mut map_lines = Vec::new();
    let mut after_map_index = 0;
    for (i, &l) in lines.iter().enumerate() {
        if l.is_empty() {
            after_map_index = i + 1;
            break;
        }
        if l.starts_with('#') {
            map_lines.push(l);
        } else {
            after_map_index = i;
            break;
        }
    }

    let mut moves_str = String::new();
    for &l in &lines[after_map_index..] {
        moves_str.push_str(l.trim());
    }

    (map_lines, moves_str)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map_lines, moves_str) = parse_input(input);

    if map_lines.is_empty() {
        return None;
    }

    let rows = map_lines.len();
    let cols = map_lines[0].len();

    let mut grid = vec![vec![Tile::Floor; cols]; rows];
    let mut robot_pos = (0i32, 0i32);

    for (r, row_str) in map_lines.iter().enumerate() {
        for (c, ch) in row_str.chars().enumerate() {
            let tile = Tile::from_char(ch);
            grid[r][c] = tile;
            if tile == Tile::Robot {
                robot_pos = (r as i32, c as i32);
            }
        }
    }

    for ch in moves_str.chars() {
        if let Some(dir) = Direction::from_char(ch) {
            let (dr, dc) = dir.delta();
            let new_r = robot_pos.0 + dr;
            let new_c = robot_pos.1 + dc;

            if out_of_bounds(new_r, new_c, rows, cols)
                || grid[new_r as usize][new_c as usize] == Tile::Wall
            {
                continue;
            }

            if grid[new_r as usize][new_c as usize] == Tile::Box
                && !push_boxes(&mut grid, new_r, new_c, dr, dc)
            {
                continue;
            }

            grid[robot_pos.0 as usize][robot_pos.1 as usize] = Tile::Floor;
            grid[new_r as usize][new_c as usize] = Tile::Robot;
            robot_pos = (new_r, new_c);
        }
    }

    let mut sum: u32 = 0;
    for (r, _) in grid.iter().enumerate().take(rows) {
        for c in 0..cols {
            if grid[r][c] == Tile::Box {
                let coord = 100 * (r as u32) + (c as u32);
                sum += coord;
            }
        }
    }
    Some(sum)
}

fn out_of_bounds(r: i32, c: i32, rows: usize, cols: usize) -> bool {
    r < 0 || r >= rows as i32 || c < 0 || c >= cols as i32
}

fn push_boxes(grid: &mut [Vec<Tile>], start_r: i32, start_c: i32, dr: i32, dc: i32) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut boxes_positions = Vec::new();
    let mut cur_r = start_r;
    let mut cur_c = start_c;
    while !out_of_bounds(cur_r, cur_c, rows, cols)
        && grid[cur_r as usize][cur_c as usize] == Tile::Box
    {
        boxes_positions.push((cur_r, cur_c));
        cur_r += dr;
        cur_c += dc;
    }

    if out_of_bounds(cur_r, cur_c, rows, cols) || grid[cur_r as usize][cur_c as usize] == Tile::Wall
    {
        return false;
    }
    if grid[cur_r as usize][cur_c as usize] == Tile::Robot {
        return false;
    }
    if grid[cur_r as usize][cur_c as usize] == Tile::Box {
        return false;
    }

    grid[cur_r as usize][cur_c as usize] = Tile::Box;
    for (i, &(br, bc)) in boxes_positions.iter().rev().enumerate() {
        if i == boxes_positions.len() - 1 {
            grid[br as usize][bc as usize] = Tile::Floor;
        } else {
            let next_pos = boxes_positions[boxes_positions.len() - 1 - (i + 1)];
            grid[br as usize][bc as usize] = Tile::Box;
            grid[next_pos.0 as usize][next_pos.1 as usize] = Tile::Floor;
        }
    }
    for i in (0..boxes_positions.len()).rev() {
        let (br, bc) = boxes_positions[i];
        grid[br as usize][bc as usize] = Tile::Floor;
        let new_r = br + dr;
        let new_c = bc + dc;
        grid[new_r as usize][new_c as usize] = Tile::Box;
    }

    true
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
