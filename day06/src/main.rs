// Advent of Code 2022 Day 06
// I rewrote this a couple times and now I can't remember why I made some of these
// decisions. I originally used windows, which is a very handy function on slices, but
// I may have just ineptly made a rough equivalent which doesn't benefit from Rust's
// iterators. Oh well.
use std::fs;

fn main() {
    // this can be really simple since we're just pulling one line out of this file
    let input_file = fs::read_to_string("input.txt").expect("The filename is probably incorrect.");

    // I can't remember why now, but converting this to bytes made it easier
    let sequence = input_file.as_bytes();

    if let Some(part_one_answer) = index_of_sequence_with_no_duplicates(sequence, 4) {
        println!("The answer for part 1 is {}.", part_one_answer + 4);
    }
    if let Some(part_two_answer) = index_of_sequence_with_no_duplicates(sequence, 14) {
        println!("The answer for part 2 is {}.", part_two_answer + 14);
    }
}

fn index_of_sequence_with_no_duplicates(sequence: &[u8], window: usize) -> Option<usize> {
    let mut current_index: usize = 0;
    loop {
        if current_index >= sequence.len() - window {
            return None;
        } else if !contains_duplicates(&sequence[current_index..=(current_index + window - 1)]) {
            return Some(current_index);
        } else {
            current_index += 1;
        }
    }
}

// I tried thinking of something like this myself, but this kind person already wrote this:
// https://stackoverflow.com/a/46766782
fn contains_duplicates(slice: &[u8]) -> bool {
    (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}
