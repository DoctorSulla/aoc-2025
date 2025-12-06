use std::fs;
use std::ops::RangeInclusive;

pub fn get_ranges(path: &str) -> Vec<RangeInclusive<u64>> {
    let file = fs::read_to_string(path).unwrap();

    file.trim()
        .split(',')
        .map(|v| {
            let (first, last) = v.split_once('-').unwrap();
            RangeInclusive::new(first.parse().unwrap(), last.parse().unwrap())
        })
        .collect()
}

pub fn add_invalid_ids(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| *range.start()..=*range.end())
        .filter(|id| id_invalid(*id))
        .sum()
}

pub fn add_invalid_ids_two(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| *range.start()..=*range.end())
        .filter(|id| id_invalid_two(*id))
        .sum()
}

fn id_invalid(id: u64) -> bool {
    let id_string: String = id.to_string();
    if id_string.len() % 2 != 0 {
        return false;
    }
    let (part_one, part_two) = id_string.split_at(id_string.len() / 2);
    part_one == part_two
}

fn id_invalid_two(id: u64) -> bool {
    let id_string: String = id.to_string();
    // Max is half the length of the string
    let max = id_string.len() / 2;
    let mut current_length = 1;

    while current_length <= max {
        let mut valid = true;
        // If it's not exactly divisible then continue
        if id_string.len() % current_length != 0 {
            current_length += 1;
            continue;
        }
        let current_chunk = &id_string[0..current_length];
        let mut pointer = current_length;
        while pointer <= id_string.len() {
            let new_chunk = &id_string[pointer - current_length..pointer];
            if new_chunk != current_chunk {
                valid = false;

                break;
            }
            pointer += current_length;
        }
        if valid {
            return valid;
        }
        current_length += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let total = add_invalid_ids(&get_ranges("./inputs/d2_test.txt"));

        assert_eq!(total, 1227775554);
    }

    #[test]
    fn p2() {
        let total = add_invalid_ids_two(&get_ranges("./inputs/d2_test.txt"));

        assert_eq!(total, 4174379265);
    }
}
