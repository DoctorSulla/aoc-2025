use std::fs;

#[derive(Default)]
struct Point {
    value: usize,
    index: usize,
}

pub fn get_joltage(file: &str) -> u64 {
    let file_text = fs::read_to_string(file).unwrap();
    let mut total = 0;

    for line in file_text.lines() {
        let mut char_one = Point::default();
        let mut char_two = Point::default();

        // First pass
        for (index, char) in line.chars().enumerate() {
            let num: usize = char.to_digit(10).unwrap() as usize;
            if num > char_one.value {
                char_one.value = num;
                char_one.index = index;
            }
        }

        if char_one.index == line.len() - 1 {
            char_two.value = char_one.value;
            char_two.index = char_one.index;
            char_one = Point::default();
            // Largest value is at the end so loop from start ignoring the last value
            for (index, char) in line.chars().enumerate() {
                let num: usize = char.to_digit(10).unwrap() as usize;
                if num > char_one.value {
                    char_one.value = num;
                    char_one.index = index;
                }
                if index == line.len() - 2 {
                    break;
                }
            }
        } else {
            for (index, char) in line.chars().enumerate().skip(char_one.index + 1) {
                let num: usize = char.to_digit(10).unwrap() as usize;
                if num > char_two.value {
                    char_two.value = num;
                    char_two.index = index;
                }
            }
        }

        let joltage: u64 = format!("{}{}", char_one.value, char_two.value)
            .parse()
            .unwrap();
        total += joltage;
    }

    total
}

pub fn get_longer_joltage(file: &str) -> u64 {
    let mut total = 0;

    let file_text = fs::read_to_string(file).unwrap();
    for line in file_text.lines() {
        let mut remaining_chars = 12;
        let mut starting_index = 0;
        let mut reading = String::new();

        while remaining_chars > 0 {
            let result = get_char(line, starting_index, remaining_chars);
            starting_index = result.1;
            remaining_chars -= 1;
            reading.push(result.0);
        }
        let reading_num = reading.parse::<u64>().unwrap();
        total += reading_num;
    }
    total
}

fn get_char(line: &str, starting_index: usize, remaining_chars: usize) -> (char, usize) {
    let mut current = Point { value: 0, index: 0 };
    for (index, char) in line.chars().enumerate().skip(starting_index) {
        if char.to_digit(10).unwrap() > current.value as u32 {
            current = Point {
                value: char.to_digit(10).unwrap() as usize,
                index: index + 1,
            };
        }
        if index == line.len() - (remaining_chars) {
            break;
        }
    }
    (
        char::from_digit(current.value as u32, 10).unwrap(),
        current.index,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let total = get_joltage("./inputs/d3_test.txt");

        assert_eq!(total, 357);
    }

    #[test]
    fn p2() {
        let total = get_longer_joltage("./inputs/d3_test.txt");

        assert_eq!(total, 3121910778619);
    }
}
