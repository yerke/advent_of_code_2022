use std::fs::read_to_string;

#[derive(Eq, PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day2.txt")?;

    let mut total_score = 0;

    input
        .split("\n")
        .for_each(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let opponent_move = translate_opponent_move(parts[0]);
            let my_move = translate_my_move(parts[1]);
            let mut score = match my_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            };
            dbg!(score);
            score += compare_moves(opponent_move, my_move);
            dbg!(score);
            total_score += score;
            dbg!(total_score);
        });

    println!("{total_score}");

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
