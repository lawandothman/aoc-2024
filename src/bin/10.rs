use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut score = 0;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 0 {
                let mut visited = HashSet::new();
                let mut queue = VecDeque::new();

                let mut reachable_nines = HashSet::new();

                queue.push_back((row, col, 0));

                visited.insert((row, col));

                while let Some((r, c, height)) = queue.pop_front() {
                    for &(dr, dc) in &directions {
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;

                        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            let next_h = grid[nr][nc];

                            if !visited.contains(&(nr, nc)) && next_h == height + 1 {
                                visited.insert((nr, nc));
                                queue.push_back((nr, nc, next_h));

                                if next_h == 9 {
                                    reachable_nines.insert((nr, nc));
                                }
                            }
                        }
                    }
                }
                score += reachable_nines.len();
            }
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut rating = 0;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 0 {
                rating += count_paths(&grid, row, col, 0, &directions, rows, cols)
            }
        }
    }

    Some(rating)
}

fn count_paths(
    grid: &[Vec<u8>],
    r: usize,
    c: usize,
    current_h: u8,
    directions: &[(isize, isize)],
    rows: usize,
    cols: usize,
) -> usize {
    let mut memo = HashMap::new();
    if current_h == 9 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(r, c, current_h)) {
        return cached;
    }

    let mut path_count = 0;

    for &(dr, dc) in directions {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            let nr = nr as usize;
            let nc = nc as usize;

            if grid[nr][nc] == current_h + 1 {
                path_count += count_paths(grid, nr, nc, current_h + 1, directions, rows, cols);
            }
        }
    }
    memo.insert((r, c, current_h), path_count);
    path_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
