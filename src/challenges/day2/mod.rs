use std::collections::HashMap;
use std::fmt;

use crate::challenges::utils::{LINE_ENDING, read_file};

pub fn run() {
    let input = read_file("src/challenges/day2/input.txt");
    let strategy = parse_strategy_guide(input.as_str());
    println!("Result phase 1 = {}", run_strategy(strategy));
}

fn parse_strategy_guide(input: &str) -> StrategyGuide {
    let sequence = input.split(LINE_ENDING).map(|line| parse_strategy_line(line)).collect();
    StrategyGuide {
        sequence
    }
}

fn parse_strategy_line(line: &str) -> Round {
    let mut elements = line.split(" ");
    let enemy = enemy_move(elements.clone().nth(0).unwrap());
    let ally = ally_move(elements.nth(1).unwrap());
    (enemy, ally)
}

fn enemy_move(m: &str) -> Move {
    match m {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissor,
        _ => panic!("Can't match enemy move"),
    }
}

fn ally_move(m: &str) -> Move {
    match m {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissor,
        _ => panic!("Can't match enemy move"),
    }
}

fn run_round(round: &Round) -> i32 {
    let enemy = &round.0;
    let ally = &round.1;

    let move_score = match ally {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    };
    let play_score = ally.run(&enemy);
    move_score + play_score
}

fn run_strategy(strategy: StrategyGuide) -> i32 {
    strategy.sequence.iter().fold(0, |score, round| score + run_round(round))
}

#[derive(Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn run(&self, other: &Self) -> i32 {
        match (self, other) {
            // win
            (Self::Rock, Self::Scissor) | (Self::Scissor, Self::Paper) | (Self::Paper, Self::Rock) => 6,
            // loss
            (Self::Scissor, Self::Rock) | (Self::Paper, Self::Scissor) | (Self::Rock, Self::Paper) => 0,
            // draw
            _ => 3,
        }
    }
}

type Round = (Move, Move);

#[derive(Debug)]
struct StrategyGuide {
    sequence: Vec<Round>,
}

#[cfg(test)]
mod day2_tests {
    use crate::challenges::day2::{parse_strategy_guide, run_strategy};
    use crate::challenges::utils::{read_file, read_input_file};

    #[test]
    fn phase_1_example() {
        let input = "A Y
B X
C Z";
        let strategy = parse_strategy_guide(input);
        println!("Hello {:?}", strategy);
        assert_eq!(run_strategy(strategy), 15);
    }
}