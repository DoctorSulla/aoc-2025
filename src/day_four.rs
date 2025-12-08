use std::{fs, isize};

struct TwoDGrid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl TwoDGrid {
    fn get_grid_value(&self, row: isize, col: isize) -> Option<&char> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;
        if row > self.height - 1 {
            return None;
        }
        if col > self.width - 1 {
            return None;
        }

        self.data.get(row * self.width + col)
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn get_accessible_rolls(file: &str) -> usize {
    let mut total = 0;

    let file_string = fs::read_to_string(file).unwrap();
    // Build grid
    let mut grid = TwoDGrid {
        width: 0,
        height: 0,
        data: vec![],
    };

    for line in file_string.lines() {
        grid.height += 1;
        grid.width = line.chars().count();

        for char in line.chars() {
            grid.data.push(char);
        }
    }

    for (index, char) in grid.data.iter().enumerate() {
        if *char != '@' {
            continue;
        }
        let row: isize = index as isize / grid.width as isize;
        let column: isize = index as isize % grid.width as isize;
        let mut adjacent_count = 0;
        for direction in DIRECTIONS {
            if grid
                .get_grid_value(row + direction.0, column + direction.1)
                .is_some_and(|v| *v == '@')
            {
                adjacent_count += 1;
            }
        }
        if adjacent_count < 4 {
            total += 1;
        }
    }

    total
}

pub fn get_rolls_repeatedly(file: &str) -> usize {
    let mut total = 0;
    let file_string = fs::read_to_string(file).unwrap();
    // Build grid
    let mut grid = TwoDGrid {
        width: 0,
        height: 0,
        data: vec![],
    };

    for line in file_string.lines() {
        grid.height += 1;
        grid.width = line.chars().count();

        for char in line.chars() {
            grid.data.push(char);
        }
    }
    while !get_accessible_row_coordinates(&grid).is_empty() {
        let coordinates = get_accessible_row_coordinates(&grid);
        total += coordinates.len();
        for coordinate in coordinates {
            grid.data[coordinate] = '.';
        }
    }
    total
}

fn get_accessible_row_coordinates(grid: &TwoDGrid) -> Vec<usize> {
    let mut coordinates = vec![];
    for (index, char) in grid.data.iter().enumerate() {
        if *char != '@' {
            continue;
        }
        let row: isize = index as isize / grid.width as isize;
        let column: isize = index as isize % grid.width as isize;
        let mut adjacent_count = 0;
        for direction in DIRECTIONS {
            if grid
                .get_grid_value(row + direction.0, column + direction.1)
                .is_some_and(|v| *v == '@')
            {
                adjacent_count += 1;
            }
        }
        if adjacent_count < 4 {
            coordinates.push(index);
        }
    }
    coordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let total = get_accessible_rolls("./inputs/d4_test.txt");

        assert_eq!(total, 13);
    }

    #[test]
    fn p2() {
        let total = get_rolls_repeatedly("./inputs/d4_test.txt");

        assert_eq!(total, 43);
    }
}
