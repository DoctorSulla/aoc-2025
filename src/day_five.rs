use std::cmp::max;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(PartialEq)]
enum States {
    CapturingRanges,
    TestingIngredients,
}

pub fn check_ingredients(file: &str) -> usize {
    let mut state = States::CapturingRanges;
    let mut fresh: HashSet<u64> = HashSet::new();
    let file_string = std::fs::read_to_string(file).unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = vec![];

    for line in file_string.lines() {
        if line.is_empty() {
            state = States::TestingIngredients;
            continue;
        }

        match state {
            States::CapturingRanges => {
                let (p1, p2) = line.split_once("-").unwrap();
                let start: u64 = p1.parse().unwrap();
                let end: u64 = p2.parse().unwrap();
                ranges.push(RangeInclusive::new(start, end));
            }
            States::TestingIngredients => {
                let ingredient_id: u64 = line.parse().unwrap();
                for range in &ranges {
                    if range.contains(&ingredient_id) {
                        fresh.insert(ingredient_id);
                    }
                }
            }
        }
    }
    fresh.len()
}

pub fn get_fresh_ranges(file: &str) -> u64 {
    let mut total = 0;
    let file_string = std::fs::read_to_string(file).unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut all_fresh: Vec<RangeInclusive<u64>> = vec![];

    for line in file_string.lines() {
        if line.is_empty() {
            break;
        }

        let (p1, p2) = line.split_once("-").unwrap();
        let start: u64 = p1.parse().unwrap();
        let end: u64 = p2.parse().unwrap();
        ranges.push(RangeInclusive::new(start, end));
    }

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    for range in ranges.into_iter() {
        if all_fresh.is_empty() {
            all_fresh.push(range)
        } else {
            let new_min = range.start();
            let new_max = range.end();

            let last = all_fresh.iter_mut().last().unwrap();

            let old_min = last.start();
            let old_max = last.end();

            if new_min <= old_max {
                *last = *old_min..=max(*new_max, *old_max);
            } else {
                all_fresh.push(range);
            }
        }
    }

    for range in all_fresh {
        total += range.end() - range.start() + 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let total = check_ingredients("./inputs/d5_test.txt");

        assert_eq!(total, 3);
    }

    #[test]
    fn p2() {
        let total = get_fresh_ranges("./inputs/d5_test.txt");

        assert_eq!(total, 14);
    }
}
