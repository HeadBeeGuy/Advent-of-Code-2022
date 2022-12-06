// Advent of Code 2022 Day 05
// This one ended up turning into quite a mess. I tend to think of how I would solve the problem
// in Ruby and then try to implement it in Rust, but there must be a bunch of features in Rust
// that I'm not using or using poorly. Juggling strings, chars, and u8s makes the code really
// noisy. I was getting so tired of it that I just have the "uncomment to solve part 2" line below,
// instead of solving both parts with the same code.
// But at least it's fast! And I suppose I learned a lot about Rust in the process.
use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Instruction {
    quantity: u8,
    start: u8,
    destination: u8,
}

fn main() {
    let input_file: Vec<String> = fs::read_to_string("input.txt")
        .expect("The filename is probably incorrect.")
        .lines()
        .map(String::from)
        .collect();
    let (mut crate_yard, instructions) = parse_input_file(&input_file);

    for instruction in instructions {
        // have to pop, store, and then push, since the compiler doesn't let me borrow two crate stacks at a time
        let mut crates_to_move: Vec<char> = vec![]; // just an intermediate storage mechanism

        if let Some(source_stack) = crate_yard.get_mut(&instruction.start) {
            for _ in 1..=instruction.quantity {
                let popped_crate = source_stack.pop().unwrap_or('?');
                crates_to_move.push(popped_crate);
            }
        }
        // uncomment to get the part 2 answer
        // crates_to_move.reverse();

        if let Some(destination_stack) = crate_yard.get_mut(&instruction.destination) {
            // crates_to_move isn't a stack - it can just be traversed in order
            for crate_char in crates_to_move {
                destination_stack.push(crate_char);
            }
        }
    }

    let crate_yard_size: u8 = crate_yard.len().try_into().unwrap_or(0);
    let mut final_message = String::new();

    for crate_stack_number in 1..=crate_yard_size {
        if let Some(current_stack) = crate_yard.get_mut(&crate_stack_number) {
            final_message.push(current_stack.pop().unwrap_or('?'));
        }
    }

    println!("The final crate message is {}", final_message);
}

fn parse_input_file(input_file: &Vec<String>) -> (HashMap<u8, Vec<char>>, Vec<Instruction>) {
    let mut crate_line_stack: Vec<String> = vec![];
    let mut initial_crate_state: HashMap<u8, Vec<char>> = HashMap::new();
    let mut instructions: Vec<Instruction> = vec![];

    let mut current_line_index: usize = 0; // keeping track of where we are in the parse, although there must be a better way

    // first, save every line we encounter with a bracket until we find no more
    for crate_definition_line in input_file.iter().take_while(|line| line.contains('[')) {
        crate_line_stack.push(crate_definition_line.clone());
        current_line_index += 1;
    }

    // next we have our line that names the crate stacks, so pop the crates on from bottom to top
    for crate_line in crate_line_stack.iter().rev() {
        // wow, I found match_indices when looking through the documentation and it's quite handy
        // but has to act on a str, so we then have to cast it to a char
        for (index, matching_char) in crate_line.match_indices(|a: char| a.is_ascii_uppercase()) {
            let stack_number: u8 = ((index + 3) / 4).try_into().unwrap_or(1);
            let char_to_insert = matching_char.chars().nth(0).unwrap_or('?');
            if initial_crate_state.contains_key(&stack_number) {
                let crate_stack = initial_crate_state
                    .get_mut(&stack_number)
                    .expect("the contains_key method didn't work.");
                crate_stack.push(char_to_insert);
            } else {
                initial_crate_state.insert(stack_number, vec![char_to_insert]);
            }
        }
    }

    current_line_index += 2; // skip the blank line before the instructions

    // populate the list of instructions
    let instruction_regex =
        Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("You messed up the regex.");

    for instruction_line in input_file[current_line_index..].iter() {
        for cap in instruction_regex.captures_iter(instruction_line) {
            let (quantity, start, destination) = (
                cap[1].parse::<u8>().unwrap_or(0),
                cap[2].parse::<u8>().unwrap_or(0),
                cap[3].parse::<u8>().unwrap_or(0),
            );

            instructions.push(Instruction {
                quantity,
                start,
                destination,
            });
        }
    }

    (initial_crate_state, instructions)
}
