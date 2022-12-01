// Advent of Code 2022 - Day 01
// I haven't touched Rust since Advent of Code 2020, I think.
// But it's still pretty neat! I think some of its lessons have sunk in over time,
// even if I can't easily recall much of the language.

use std::fs;
fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the input file.");

    // each elf's inventory is separated by two line breaks (which creates the single blank line we see in the file)
    // then each inventory is separated by a single line break

    let mut sums: Vec<usize> = input_file
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split("\n")
                .map(|a| a.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect();

    sums.sort();
    sums.reverse();

    println!(
        "The most calories carried by any one elf is {}.",
        &sums.first().unwrap_or(&0)
    );

    let top_three_calorie_total = &sums[0..=2].iter().sum::<usize>();
    println!(
        "The sum of the three highest calorie meals is {}.",
        top_three_calorie_total
    );
}
