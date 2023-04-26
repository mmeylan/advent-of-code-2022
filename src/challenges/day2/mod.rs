use crate::challenges::utils::{read_file, LINE_ENDING};

pub fn run() {
    let input = read_file("src/challenges/day2/input.txt");
    let strategy = StrategyGuideP1 {};
    println!("Result phase 1 = {}", strategy.run(input.as_str()));
    let strategy2: StrategyGuideP2 = StrategyGuideP2 {};
    println!("Result phase 2 = {}", strategy2.run(input.as_str()));
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
            (Self::Rock, Self::Scissor)
            | (Self::Scissor, Self::Paper)
            | (Self::Paper, Self::Rock) => 6,
            // loss
            (Self::Scissor, Self::Rock)
            | (Self::Paper, Self::Scissor)
            | (Self::Rock, Self::Paper) => 0,
            // draw
            _ => 3,
        }
    }
}

type Round = (Move, Move);

fn play(round: &Round) -> i32 {
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

trait StrategyGuide {
    fn run(&self, strategy_script: &str) -> i32;

    fn parse_command<'a>(&self, line: &'a str) -> (&'a str, &'a str) {
        let mut elements = line.split(" ");
        let enemy = elements.clone().nth(0).unwrap();
        let ally = elements.nth(1).unwrap();
        (enemy, ally)
    }
}

struct StrategyGuideP1 {}

impl StrategyGuideP1 {
    fn enemy_move(&self, m: &str) -> Move {
        match m {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _ => panic!("Can't match enemy move"),
        }
    }

    fn ally_move(&self, m: &str) -> Move {
        match m {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissor,
            _ => panic!("Can't match enemy move"),
        }
    }

    fn round(&self, enemy: &str, ally: &str) -> Round {
        (self.enemy_move(enemy), self.ally_move(ally))
    }
}

impl StrategyGuide for StrategyGuideP1 {
    fn run(&self, strategy_script: &str) -> i32 {
        strategy_script
            .split(LINE_ENDING)
            .map(|line| self.parse_command(line))
            .map(|(enemy, ally)| self.round(enemy, ally))
            .fold(0, |score, round| score + play(&round))
    }
}

struct StrategyGuideP2 {}

impl StrategyGuideP2 {
    fn enemy_move(&self, m: &str) -> Move {
        match m {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _ => panic!("Can't match enemy move"),
        }
    }

    fn ally_move(&self, m: &str, enemy_move: &Move) -> Move {
        match m {
            "X" => self.lose(enemy_move),
            "Y" => self.tie(enemy_move),
            "Z" => self.win(enemy_move),
            _ => panic!("Can't match enemy move"),
        }
    }

    fn lose(&self, enemy_move: &Move) -> Move {
        match enemy_move {
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissor,
            Move::Scissor => Move::Paper,
        }
    }

    fn win(&self, enemy_move: &Move) -> Move {
        match enemy_move {
            Move::Paper => Move::Scissor,
            Move::Rock => Move::Paper,
            Move::Scissor => Move::Rock,
        }
    }

    fn tie(&self, enemy_move: &Move) -> Move {
        return enemy_move.clone();
    }

    fn round(&self, enemy: &str, ally: &str) -> Round {
        let enemy_move = self.enemy_move(enemy);
        let ally_move = self.ally_move(ally, &enemy_move);
        (enemy_move, ally_move)
    }
}

impl StrategyGuide for StrategyGuideP2 {
    fn run(&self, strategy_script: &str) -> i32 {
        strategy_script
            .split(LINE_ENDING)
            .map(|line| self.parse_command(line))
            .map(|(enemy, ally)| self.round(enemy, ally))
            .fold(0, |score, round| score + play(&round))
    }
}

#[cfg(test)]
mod day2_tests {
    use crate::challenges::day2::{StrategyGuide, StrategyGuideP1, StrategyGuideP2};

    #[test]
    fn phase_1_example() {
        let input = "A Y
B X
C Z";
        let strategy = StrategyGuideP1 {};
        let score = strategy.run(input);
        assert_eq!(score, 15);
    }

    #[test]
    fn phase_2_example() {
        let input = "A Y
B X
C Z";
        let strategy = StrategyGuideP2 {};
        let score = strategy.run(input);
        assert_eq!(score, 12);
    }
}
