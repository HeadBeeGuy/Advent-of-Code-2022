use std::fs;

enum Move {
    Rock,
    Scissors,
    Paper,
}

enum GameResult {
    PlayerWins,
    OpponentWins,
    Draw,
}

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the input file.");
    let games = input_file.split('\n');

    let mut part_one_score: usize = 0;
    let mut part_two_score: usize = 0;

    for game in games {
        if let (Some(a), Some(b)) = (&game.chars().nth(0), &game.chars().nth(2)) {
            let opponent_move = determine_move(a);

            // part 1 calculation
            let part_one_player_move = determine_move(b);
            part_one_score += player_score_for_move(&part_one_player_move);

            let part_one_result = result_of_game(&opponent_move, &part_one_player_move);
            part_one_score += player_score_for_game_result(&part_one_result);

            // part 2 calculation
            let desired_outcome = intended_outcome(b);
            let part_two_player_move =
                player_move_to_reach_intended_outcome(&opponent_move, &desired_outcome);

            part_two_score += player_score_for_move(&part_two_player_move);
            part_two_score += player_score_for_game_result(&desired_outcome);
        }
    }

    println!("Your final score for part one was {}.", part_one_score);
    println!("Your final score for part two was {}.", part_two_score);
}

fn determine_move(code: &char) -> Move {
    match code {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        // 'C' | 'Z' => Move::Scissors,
        _ => Move::Scissors, // scissors by default
    }
}

fn result_of_game(opponent_move: &Move, player_move: &Move) -> GameResult {
    match (opponent_move, player_move) {
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => GameResult::Draw,
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => GameResult::PlayerWins,
        (Move::Rock, Move::Scissors)
        | (Move::Scissors, Move::Paper)
        | (Move::Paper, Move::Rock) => GameResult::OpponentWins,
    }
}

fn player_score_for_move(player_move: &Move) -> usize {
    match player_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn player_score_for_game_result(game_result: &GameResult) -> usize {
    match game_result {
        GameResult::OpponentWins => 0,
        GameResult::Draw => 3,
        GameResult::PlayerWins => 6,
    }
}

fn intended_outcome(code: &char) -> GameResult {
    match code {
        'X' => GameResult::OpponentWins,
        'Y' => GameResult::Draw,
        _ => GameResult::PlayerWins,
    }
}

fn player_move_to_reach_intended_outcome(
    opponent_move: &Move,
    intended_outcome: &GameResult,
) -> Move {
    match (opponent_move, intended_outcome) {
        (Move::Rock, GameResult::PlayerWins) => Move::Paper,
        (Move::Rock, GameResult::OpponentWins) => Move::Scissors,
        (Move::Rock, GameResult::Draw) => Move::Rock,
        (Move::Scissors, GameResult::PlayerWins) => Move::Rock,
        (Move::Scissors, GameResult::OpponentWins) => Move::Paper,
        (Move::Scissors, GameResult::Draw) => Move::Scissors,
        (Move::Paper, GameResult::PlayerWins) => Move::Scissors,
        (Move::Paper, GameResult::OpponentWins) => Move::Rock,
        (Move::Paper, GameResult::Draw) => Move::Paper,
    }
}
