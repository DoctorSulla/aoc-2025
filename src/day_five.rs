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
                for range in ranges.clone() {
                    for num in range {
                        if num == ingredient_id {
                            fresh.insert(num);
                        }
                    }
                }
            }
        }
    }
    fresh.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let total = check_ingredients("./inputs/d5_test.txt");

        assert_eq!(total, 3);
    }
}
