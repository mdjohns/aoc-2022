use std::{fs, ops::Add};

#[derive(Debug)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Move {
    fn points(self) -> i32 {
        match self {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        }
    }
}

enum Outcome {
    WIN,
    LOSS,
    DRAW,
}

impl Outcome {
    fn points(self) -> i32 {
        match self {
            Outcome::WIN => 6,
            Outcome::LOSS => 0,
            Outcome::DRAW => 3,
        }
    }
}

fn map_opponent_move(opponent_move: &str) -> Result<Move, &'static str> {
    match opponent_move {
        "A" => Ok(Move::ROCK),
        "B" => Ok(Move::PAPER),
        "C" => Ok(Move::SCISSORS),
        _ => Err("invalid move"),
    }
}

fn map_player_move(player_move: &str) -> Result<Move, &'static str> {
    match player_move {
        "X" => Ok(Move::ROCK),
        "Y" => Ok(Move::PAPER),
        "Z" => Ok(Move::SCISSORS),
        _ => Err("invalid move"),
    }
}

fn map_desired_outcome(desired_outcome: &str) -> Result<Outcome, &'static str> {
    match desired_outcome {
        "X" => Ok(Outcome::LOSS),
        "Y" => Ok(Outcome::DRAW),
        "Z" => Ok(Outcome::WIN),
        _ => Err("invalid outcome"),
    }
}

fn map_moves(round: &str) -> (Move, Move) {
    let (opponent, player) = round.split_once(" ").unwrap();

    (
        map_opponent_move(opponent).unwrap(),
        map_player_move(player).unwrap(),
    )
}

fn map_move_and_desired_outcome(round: &str) -> (Move, Outcome) {
    let (opponent, desired_outcome) = round.split_once(" ").unwrap();

    (
        map_opponent_move(opponent).unwrap(),
        map_desired_outcome(desired_outcome).unwrap(),
    )
}

fn map_desired_outcome_to_player_move(
    (opponent_move, desired_outcome): (Move, Outcome),
) -> (Move, Move) {
    let player_move = match desired_outcome {
        Outcome::WIN => match opponent_move {
            Move::ROCK => Move::PAPER,
            Move::PAPER => Move::SCISSORS,
            Move::SCISSORS => Move::ROCK,
        },
        Outcome::DRAW => match opponent_move {
            Move::ROCK => Move::ROCK,
            Move::PAPER => Move::PAPER,
            Move::SCISSORS => Move::SCISSORS,
        },
        Outcome::LOSS => match opponent_move {
            Move::ROCK => Move::SCISSORS,
            Move::PAPER => Move::ROCK,
            Move::SCISSORS => Move::PAPER,
        },
    };

    (opponent_move, player_move)
}

fn score_round((opponent_move, player_move): (Move, Move)) -> i32 {
    match opponent_move {
        Move::ROCK => match player_move {
            Move::ROCK => Outcome::DRAW.points().add(Move::ROCK.points()),
            Move::PAPER => Outcome::WIN.points().add(Move::PAPER.points()),
            Move::SCISSORS => Outcome::LOSS.points().add(Move::SCISSORS.points()),
        },
        Move::PAPER => match player_move {
            Move::ROCK => Outcome::LOSS.points().add(Move::ROCK.points()),
            Move::PAPER => Outcome::DRAW.points().add(Move::PAPER.points()),
            Move::SCISSORS => Outcome::WIN.points().add(Move::SCISSORS.points()),
        },
        Move::SCISSORS => match player_move {
            Move::ROCK => Outcome::WIN.points().add(Move::ROCK.points()),
            Move::PAPER => Outcome::LOSS.points().add(Move::PAPER.points()),
            Move::SCISSORS => Outcome::DRAW.points().add(Move::SCISSORS.points()),
        },
    }
}

fn part_1(file_str: &str) -> i32 {
    file_str
        .split("\n")
        .map(|round| map_moves(round))
        .map(|moves| score_round(moves))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn part_2(file_str: &str) -> i32 {
    file_str
        .split("\n")
        .map(|round| map_move_and_desired_outcome(round))
        .map(|move_and_outcome| map_desired_outcome_to_player_move(move_and_outcome))
        .map(|moves| score_round(moves))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn main() {
    let file_contents = fs::read_to_string("../puzzles/day2.txt").unwrap();
    let file_str = file_contents.trim();

    let first_answer = part_1(file_str);
    println!("First answer: {}", first_answer);

    let second_answer = part_2(file_str);
    println!("Second answer: {}", second_answer);
}
