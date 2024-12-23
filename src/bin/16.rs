use std::collections::{BinaryHeap, VecDeque};

advent_of_code::solution!(16);

#[derive(Eq, PartialEq)]
struct State {
    cost: u32,
    r: usize,
    c: usize,
    d: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // min-heap by cost
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn compute_distances(
    grid: &[Vec<char>],
    start_states: &[(usize, usize, usize, u32)],
) -> Vec<Vec<Vec<u32>>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut dist = vec![vec![vec![u32::MAX; 4]; w]; h];

    let mut heap = BinaryHeap::new();

    for &(sr, sc, sd, cost) in start_states {
        dist[sr][sc][sd] = cost;
        heap.push(State {
            cost,
            r: sr,
            c: sc,
            d: sd,
        });
    }

    while let Some(State { cost, r, c, d }) = heap.pop() {
        if cost > dist[r][c][d] {
            continue;
        }
        let (dr, dc) = DIRECTIONS[d];
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] != '#' {
                let new_cost = cost + 1;
                if new_cost < dist[nr][nc][d] {
                    dist[nr][nc][d] = new_cost;
                    heap.push(State {
                        cost: new_cost,
                        r: nr,
                        c: nc,
                        d,
                    });
                }
            }
        }
        for &new_d in &[(d + 3) % 4, (d + 1) % 4] {
            let new_cost = cost + 1000;
            if new_cost < dist[r][c][new_d] {
                dist[r][c][new_d] = new_cost;
                heap.push(State {
                    cost: new_cost,
                    r,
                    c,
                    d: new_d,
                });
            }
        }
    }

    dist
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (h, w) = (grid.len(), grid[0].len());

    let mut start = None;
    let mut end = None;
    for (r, _) in grid.iter().enumerate().take(h) {
        for c in 0..w {
            match grid[r][c] {
                'S' => start = Some((r, c)),
                'E' => end = Some((r, c)),
                _ => {}
            }
        }
    }
    let (sr, sc) = start?;
    let (er, ec) = end?;

    let dist_from_start = compute_distances(&grid, &[(sr, sc, 0, 0)]);

    let best_cost = (0..4)
        .map(|d| dist_from_start[er][ec][d])
        .min()
        .unwrap_or(u32::MAX);

    if best_cost == u32::MAX {
        None
    } else {
        Some(best_cost)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (h, w) = (grid.len(), grid[0].len());

    let (mut sr, mut sc) = (0, 0);
    let (mut er, mut ec) = (0, 0);
    for (r, _) in grid.iter().enumerate().take(h) {
        for c in 0..w {
            match grid[r][c] {
                'S' => {
                    sr = r;
                    sc = c;
                }
                'E' => {
                    er = r;
                    ec = c;
                }
                _ => {}
            }
        }
    }

    let dist = compute_distances(&grid, &[(sr, sc, 0, 0)]);

    let best_cost = (0..4).map(|d| dist[er][ec][d]).min().unwrap_or(u32::MAX);
    if best_cost == u32::MAX {
        return None;
    }

    let mut on_path = vec![vec![false; w]; h];
    let mut visited = vec![vec![vec![false; 4]; w]; h];

    let mut queue = VecDeque::new();
    for d in 0..4 {
        if dist[er][ec][d] == best_cost {
            visited[er][ec][d] = true;
            queue.push_back((er, ec, d));
        }
    }

    while let Some((r, c, d)) = queue.pop_front() {
        on_path[r][c] = true;
        let cost_here = dist[r][c][d];

        if cost_here >= 1 {
            let (dr, dc) = DIRECTIONS[d];
            let nr = (r as isize - dr) as usize;
            let nc = (c as isize - dc) as usize;
            if nr < h && nc < w && dist[nr][nc][d] == cost_here - 1 && !visited[nr][nc][d] {
                visited[nr][nc][d] = true;
                queue.push_back((nr, nc, d));
            }
        }

        if cost_here >= 1000 {
            for &nd in &[(d + 3) % 4, (d + 1) % 4] {
                if dist[r][c][nd] == cost_here - 1000 && !visited[r][c][nd] {
                    visited[r][c][nd] = true;
                    queue.push_back((r, c, nd));
                }
            }
        }
    }

    let count = on_path
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum::<usize>() as u32;

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
