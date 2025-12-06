use std::fs;

pub fn get_num_zeroes(file: &str) -> u16 {
    let mut zeroes = 0;
    let mut dial_value: i16 = 50;

    let file = fs::read_to_string(&file).unwrap();

    for line in file.lines() {
        let (direction, amount) = line.split_at(1);
        let amount: i16 = amount.parse().unwrap();
        if direction == "R" {
            dial_value += amount % 100;
            if dial_value > 99 {
                dial_value -= 100;
            }
        } else {
            dial_value -= amount % 100;
            if dial_value < 0 {
                dial_value = 100 + dial_value;
            }
        }
        if dial_value == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

pub fn get_num_zeroes_two(file: &str) -> i16 {
    let mut zeroes = 0;
    let mut dial_value: i16 = 50;

    let file = fs::read_to_string(&file).unwrap();

    for line in file.lines() {
        let (direction, amount) = line.split_at(1);
        let amount: i16 = amount.parse().unwrap();

        let remainder = amount % 100;
        let full_rotations = (amount - remainder) / 100;
        zeroes += full_rotations;

        if remainder == 0 {
            continue;
        }

        if direction == "R" {
            dial_value += remainder;
            if dial_value > 99 {
                zeroes += 1;
                dial_value -= 100;
            }
        } else {
            let old_dial = dial_value;
            dial_value -= remainder;
            if dial_value < 0 {
                dial_value += 100;
                if old_dial != 0 {
                    zeroes += 1;
                }
            } else if dial_value == 0 {
                zeroes += 1;
            }
        }
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = get_num_zeroes("./inputs/d1_test.txt");

        assert_eq!(result, 3);
    }

    #[test]
    fn p2() {
        let result = get_num_zeroes_two("./inputs/d1_test.txt");

        assert_eq!(result, 6);
    }
}
