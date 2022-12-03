// Advent of Code 2022 Day 03
// As with Day 2, rather verbose. But it was good to get more practice with if let and optionals.

use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the input file.");
    let rucksacks: Vec<&str> = input_file.split('\n').collect();
    let mut part_one_sum: usize = 0;
    let mut part_two_sum: usize = 0;

    // not especially clean, but it looks like Rust doesn't let you build ranges with chars
    let mut priorities = HashMap::new();
    for (index, ch) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
    {
        priorities.insert(ch, index + 1);
    }

    for items in rucksacks.chunks(3) {
        if let (Some(first), Some(second), Some(third)) = (items.get(0), items.get(1), items.get(2))
        {
            // not very elegant. I did part 2 by just running part 1's calculations in these chunks of three, and then...
            part_one_sum += priority_score_for_rucksack(first, &priorities).unwrap_or(0);
            part_one_sum += priority_score_for_rucksack(second, &priorities).unwrap_or(0);
            part_one_sum += priority_score_for_rucksack(third, &priorities).unwrap_or(0);

            // doing the part 2 calculation.
            part_two_sum +=
                priority_score_for_triple_match(first, second, third, &priorities).unwrap_or(0);
        }
    }
    println!(
        "The answer for part 1 (finding duplicates in each rucksack) is {}.",
        part_one_sum
    );
    println!(
        "The answer for part 2 (finding duplicates across three rucksacks) is {}.",
        part_two_sum
    );
}

// passing priorities in just so I don't have to construct it every time I call this function.
// I think I heard somewhere that globally-scoped variables are frowned upon in Rust.
fn priority_score_for_rucksack(rucksack: &str, priorities: &HashMap<char, usize>) -> Option<usize> {
    let rucksack_size = rucksack.len(); // the problem seems to assume it's always even

    if rucksack_size > 0 {
        // the end of file will be an empty newline
        let (first_half, second_half) = rucksack.split_at(rucksack_size / 2);

        for character in first_half.chars() {
            if second_half.contains(character) {
                if let Some(priority) = priorities.get(&character) {
                    return Some(*priority);
                }
            }
        }
    }
    return None; // hey, how 'bout that! This is the first time I've actually generated an Option
}

fn priority_score_for_triple_match(
    first_rucksack: &str,
    second_rucksack: &str,
    third_rucksack: &str,
    priorities: &HashMap<char, usize>,
) -> Option<usize> {
    for character in first_rucksack.chars() {
        if second_rucksack.contains(character) && third_rucksack.contains(character) {
            if let Some(priority) = priorities.get(&character) {
                return Some(*priority);
            }
        }
    }
    return None;
}
