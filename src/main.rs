use aoc2025::day_one::*;
use aoc2025::day_three::*;
use aoc2025::day_two::*;

fn main() {
    println!("Day 1 Part 1 answer: {}", get_num_zeroes("./inputs/d1.txt"));
    println!(
        "Day 1 Part 2 answer: {}",
        get_num_zeroes_two("./inputs/d1.txt")
    );

    println!(
        "Day 2 Part 1 answer: {}",
        add_invalid_ids(&get_ranges("./inputs/d2.txt"))
    );

    println!(
        "Day 2 Part 2 answer: {}",
        add_invalid_ids_two(&get_ranges("./inputs/d2.txt"))
    );

    println!("Day 3 Part 1 answer: {}", get_joltage("./inputs/d3.txt"));
    println!(
        "Day 3 Part 2 answer: {}",
        get_longer_joltage("./inputs/d3.txt")
    );
}
