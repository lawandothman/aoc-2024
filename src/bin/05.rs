use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_section, updates_section) = input.split_once("\n\n")?;

    let mut rules: Vec<(u32, u32)> = Vec::new();

    for rule in rules_section.lines() {
        let (x, y) = rule.split_once("|")?;
        rules.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    let updates: Vec<Vec<u32>> = updates_section
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .map(|n| n.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()?;

    let mut sum = 0;

    for update in updates {
        if is_ordered(&update, &rules) {
            let m_idx = update.len() / 2;
            let m_page = update[m_idx];
            sum += m_page;
        }
    }

    Some(sum)
}

fn is_ordered(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let mut page_indices: HashMap<u32, usize> = HashMap::new();
    for (i, page) in update.iter().enumerate() {
        page_indices.insert(*page, i);
    }

    for &(x, y) in rules {
        if let (Some(x_idx), Some(y_idx)) = (page_indices.get(&x), page_indices.get(&y)) {
            if x_idx >= y_idx {
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_section, updates_section) = input.split_once("\n\n")?;

    let mut rules: Vec<(u32, u32)> = Vec::new();

    for rule in rules_section.lines() {
        let (x, y) = rule.split_once("|")?;
        rules.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    let updates: Vec<Vec<u32>> = updates_section
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .map(|n| n.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()?;

    let mut sum = 0;

    for update in updates {
        if !is_ordered(&update, &rules) {
            if let Some(corrected) = reorder(&update, &rules) {
                let m_idx = corrected.len() / 2;
                let m_page = corrected[m_idx];
                sum += m_page;
            } else {
                continue;
            }
        }
    }
    Some(sum)
}

fn reorder(update: &[u32], rules: &[(u32, u32)]) -> Option<Vec<u32>> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    for &page in update {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    for &(before, after) in rules {
        if update.contains(&before) && update.contains(&after) {
            graph.entry(before).or_default().push(after);
            *in_degree.entry(after).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<u32> = VecDeque::new();
    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted_pages = Vec::new();

    while let Some(page) = queue.pop_front() {
        sorted_pages.push(page);

        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    if sorted_pages.len() == update.len() {
        let page_set: HashSet<u32> = update.iter().cloned().collect();
        let sorted_update: Vec<u32> = sorted_pages
            .into_iter()
            .filter(|page| page_set.contains(page))
            .collect();
        Some(sorted_update)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
