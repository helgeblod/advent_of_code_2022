extern crate core;

use std::fs;
use crate::ShapeType::Rock;
use crate::ShapeType::Paper;
use crate::ShapeType::Scissors;

const ROCK: Shape = Shape {
    shape_type: Rock,
    beats: Scissors,
    looses_against: Paper,
    bonus_points: 1
};
const PAPER: Shape = Shape {
    shape_type: Paper,
    beats: Rock,
    looses_against: Scissors,
    bonus_points: 2
};
const SCISSORS: Shape = Shape {
    shape_type: Scissors,
    beats: Paper,
    looses_against: Rock,
    bonus_points: 3
};

pub struct Shape {
    shape_type: ShapeType,
    beats: ShapeType,
    looses_against: ShapeType,
    bonus_points: i32,
}

impl Combat for Shape {
    fn score_battle(&self, opponent: &Shape) -> i32 {
        let outcome_points = if self.beats == opponent.shape_type {
            Outcome::Win as i32
        } else if self.shape_type == opponent.shape_type {
            Outcome::Draw as i32
        } else { Outcome::Loose as i32 };

        self.bonus_points + outcome_points 
    }
}

#[derive(PartialEq, Eq)]
pub enum ShapeType {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win = 6,
    Draw = 3,
    Loose = 0,
}

pub trait Combat {
    fn score_battle(&self, opponent: &Shape) -> i32;
}

fn main() {
    let strategy_input = fs::read_to_string("./src/strategy-input.txt").expect("Could not open strategy file");
    println!("🏆 My total score part 1: {}", part1(&strategy_input));
    println!("🏆 My total score part 2: {}", part2(&strategy_input));
}

fn part1(strategy_input: &str) -> i32 {
    let mut total_points = 0i32;
    // Parse input 
    for line in strategy_input.lines() {
        let strategies: Vec<&str> = line.split_whitespace().collect();
        let opponent_strategy = strategy_matcher(strategies[0]).expect("Unknown strategy"); // May panic
        let my_strategy = strategy_matcher(strategies[1]).expect("Unknown strategy"); // May panic
        total_points += my_strategy.score_battle(&opponent_strategy);
    }
    total_points
}

fn part2(strategy_input: &str) -> i32 {
    let mut total_points = 0i32;
    for line in strategy_input.lines() {
        let strategies: Vec<&str> = line.split_whitespace().collect();
        let opponent_strategy = strategy_matcher(strategies[0]).expect("Unknown strategy"); // May panic
        let my_strategy = result_matcher(&opponent_strategy, strategies[1]).expect("Unknown strategy"); // May panic
        total_points += my_strategy.score_battle(&opponent_strategy);
    }
    total_points
}

fn strategy_matcher(strategy: &str) -> Option<Shape> {
    match strategy {
        "A" => Some(ROCK),
        "B" => Some(PAPER),
        "C" => Some(SCISSORS),
        "X" => Some(ROCK),
        "Y" => Some(PAPER),
        "Z" => Some(SCISSORS),
        &_ => None
    }
}

fn result_matcher(opponent_shape: &Shape, expected_result: &str) -> Option<Shape> {
    let my_shape_type = match expected_result {
        "X" => &opponent_shape.beats, // loose
        "Y" => &opponent_shape.shape_type, // draw
        "Z" => &opponent_shape.looses_against, // win
        &_ => panic!("Unknown result strategy")
    };
    
    match my_shape_type {
        Rock => Some(ROCK),
        Paper => Some(PAPER),
        Scissors => Some(SCISSORS),
    } 
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    const STRATEGY_INPUT: &str = indoc! {"A Y
                                          B X
                                          C Z"};
    #[test]
    fn test_part1() {
        let my_points = part1(STRATEGY_INPUT);
        assert_eq!(my_points, 15);
    }
    
    #[test]
    fn test_part2() {
        let my_points = part2(STRATEGY_INPUT);
        assert_eq!(my_points, 12);
    }
}
