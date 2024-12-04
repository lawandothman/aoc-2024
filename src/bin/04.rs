advent_of_code::solution!(4);

fn check_word_part_one(
    grid: &[Vec<char>],
    needle: &[char],
    mut x: isize,
    mut y: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let needle_len = needle.len() as isize;

    for i in 0..needle_len {
        if x < 0
            || x >= rows
            || y < 0
            || y >= cols
            || grid[x as usize][y as usize] != needle[i as usize]
        {
            return false;
        }
        x += dx;
        y += dy;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();

    if rows == 0 {
        return Some(0);
    }

    let cols = grid[0].len();
    let needle: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;

    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down Right
        (1, -1),  // Down Left
        (-1, 1),  // Up Right
        (-1, -1), // Up Left
    ];

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions {
                if check_word_part_one(&grid, &needle, x as isize, y as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn matches_mas(seq: &[char; 3]) -> bool {
    *seq == ['M', 'A', 'S'] || *seq == ['S', 'A', 'M']
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    if rows < 3 {
        return Some(0);
    }

    let cols = grid[0].len();
    if cols < 3 {
        return Some(0);
    }

    let mut count = 0;

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            let c1 = grid[x - 1][y - 1];
            let c2 = grid[x][y];
            let c3 = grid[x + 1][y + 1];
            let c4 = grid[x - 1][y + 1];
            let c5 = grid[x + 1][y - 1];

            let seq1 = [c1, c2, c3];
            let seq2 = [c4, c2, c5];

            if matches_mas(&seq1) && matches_mas(&seq2) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
