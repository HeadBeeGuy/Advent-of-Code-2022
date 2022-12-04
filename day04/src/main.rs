// Advent of Code 2022 Day 04
// Just an unsophisticated parse, but it's enough.
use regex::Regex;
use std::fs;

fn main() {
    let pair_regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("You messed up the Regex!");

    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the input file.");

    let lines = input_file.split('\n');
    let mut ranges_that_contain_another: usize = 0;
    let mut overlapping_ranges: usize = 0;

    for line in lines {
        for cap in pair_regex.captures_iter(line) {
            let (start1, end1, start2, end2) = (
                cap[1].parse::<u32>().unwrap_or(0),
                cap[2].parse::<u32>().unwrap_or(0),
                cap[3].parse::<u32>().unwrap_or(0),
                cap[4].parse::<u32>().unwrap_or(0),
            );

            // part 1
            if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
                ranges_that_contain_another += 1;
            }

            // part 2
            // I got the math mixed up when I was trying to write this myself, so this is just from this post:
            // https://stackoverflow.com/a/3269471
            if (start1 <= end2) && (end1 >= start2) {
                overlapping_ranges += 1;
            }
        }
    }

    println!(
        "There were {} ranges that contained another range.",
        ranges_that_contain_another
    );
    println!("There were {} overlapping ranges.", overlapping_ranges);
}
