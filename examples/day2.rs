use std::fs::read_to_string;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Debug)]
enum GameResult {
    Win,
    Lose,
    Draw
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day2.txt")?;

    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;

    input
        .split("\n")
        .for_each(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let opponent_move = translate_opponent_move(parts[0]);
            let my_move = translate_my_move(parts[1]);
            let mut score_part1 = my_move_score(&my_move);
            score_part1 += compare_moves(opponent_move, my_move);
            total_score_part1 += score_part1;

            // part 2
            let expected_result = translate_expected_result(parts[1]);
            let my_expected_move = get_move(&opponent_move, expected_result);
            let mut score_part2 = my_move_score(&my_expected_move);
            score_part2 += compare_moves(opponent_move, my_expected_move);
            total_score_part2 += score_part2;
        });

    println!("{total_score_part1}");
    println!("{total_score_part2}");

    Ok(())
}

fn translate_opponent_move(s: &str) -> Move {
    match s {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Unexpected opponent move: {s}")
    }
}

fn translate_my_move(s: &str) -> Move {
    match s {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Unexpected my move: {s}")
    }
}

fn translate_expected_result(s: &str) -> GameResult {
    match s {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("Unexpected expected result: {s}")
    }
}

fn get_move(opponent: &Move, expected_result: GameResult) -> Move {
    if expected_result == GameResult::Draw {
        return opponent.clone();
    }

    match (opponent, expected_result) {
        (Move::Rock, GameResult::Win) => Move::Paper,
        (Move::Rock, GameResult::Lose) => Move::Scissors,
        (Move::Paper, GameResult::Win) => Move::Scissors,
        (Move::Paper, GameResult::Lose) => Move::Rock,
        (Move::Scissors, GameResult::Win) => Move::Rock,
        (Move::Scissors, GameResult::Lose) => Move::Paper,
        _ => panic!("expected draw should have been handled earlier")
    }
}

fn my_move_score(my_move: &Move) -> i32 {
    match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn compare_moves(opponent: Move, mine: Move) -> i32 {
    if opponent == mine {
        return 3; // draw
    }

    match (opponent, mine) {
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        _ => panic!("draw should have been handled earlier")
    }
}
