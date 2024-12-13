use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

struct Map {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

struct Region {
    plant: char,
    coords: HashSet<(usize, usize)>,
    area: usize,
}

fn voronoi(grid: &[Vec<char>], rows: usize, cols: usize) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for row in 0..rows {
        for col in 0..cols {
            if visited.contains(&(row, col)) {
                continue;
            }

            let plant = grid[row][col];
            let mut coords = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((row, col));

            while let Some((r, c)) = queue.pop_front() {
                if visited.contains(&(r, c)) {
                    continue;
                }

                visited.insert((r, c));
                coords.insert((r, c));

                for (dr, dc) in &directions {
                    let new_r = (r as i32 + dr) as usize;
                    let new_c = (c as i32 + dc) as usize;

                    if new_r < rows
                        && new_c < cols
                        && grid[new_r][new_c] == plant
                        && !visited.contains(&(new_r, new_c))
                    {
                        queue.push_back((new_r, new_c));
                    }
                }
            }

            regions.push(Region {
                plant,
                area: coords.len(),
                coords,
            });
        }
    }

    regions
}

pub fn part_one(input: &str) -> Option<usize> {
    let max_col = input.lines().next().unwrap().len();
    let mut max_row = 0;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            max_row += 1;
            line.chars().collect()
        })
        .collect();

    let regions = voronoi(&grid, max_row, max_col);
    let map = Map {
        grid,
        rows: max_row,
        cols: max_col,
    };
    let mut total = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for region in regions {
        let mut perimeter = 0;
        for &(r, c) in &region.coords {
            for (dr, dc) in &directions {
                let new_r = (r as i32 + dr) as usize;
                let new_c = (c as i32 + dc) as usize;

                if new_r >= map.rows || new_c >= map.cols || map.grid[new_r][new_c] != region.plant
                {
                    perimeter += 1;
                }
            }
        }

        total += region.coords.len() * perimeter;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let max_col = input.lines().next().unwrap().len();
    let mut max_row = 0;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            max_row += 1;
            line.chars().collect()
        })
        .collect();

    let regions = voronoi(&grid, max_row, max_col);
    let map = Map {
        grid,
        rows: max_row,
        cols: max_col,
    };

    let mut total = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for region in regions {
        let mut edges = HashSet::new();
        for &(r, c) in &region.coords {
            for (idx, (dr, dc)) in directions.iter().enumerate() {
                let new_r = (r as i32 + dr) as usize;
                let new_c = (c as i32 + dc) as usize;

                if new_r >= map.rows || new_c >= map.cols || map.grid[new_r][new_c] != region.plant
                {
                    edges.insert((r, c, idx));
                }
            }
        }

        let mut sides = 0;
        let mut visited_edges = HashSet::new();

        for &(r, c, dir) in &edges {
            if visited_edges.contains(&(r, c, dir)) {
                continue;
            }

            sides += 1;

            let perp_dirs = match dir {
                0 | 2 => [(1, 0), (-1, 0)],
                1 | 3 => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for &(dr, dc) in &perp_dirs {
                let mut curr_r = r as i32;
                let mut curr_c = c as i32;

                loop {
                    curr_r += dr;
                    curr_c += dc;

                    if curr_r < 0
                        || curr_r >= map.rows as i32
                        || curr_c < 0
                        || curr_c >= map.cols as i32
                    {
                        break;
                    }

                    let curr_coord = (curr_r as usize, curr_c as usize, dir);
                    if !edges.contains(&curr_coord) {
                        break;
                    }

                    visited_edges.insert(curr_coord);
                }
            }
        }

        total += region.area * sides;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
