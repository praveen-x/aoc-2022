use std::fs::read_to_string;
use strum::EnumString;

use Move::*;
use WhatToDo::*;

#[derive(Debug, PartialEq, EnumString, Copy, Clone)]
enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

#[derive(Debug, PartialEq, EnumString, Copy, Clone)]
enum WhatToDo {
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Round {
    enemy: Move,
    you: Move,
}

impl Round {
    fn from_part_1(s: &str) -> Self {
        let mut moves = s.split_whitespace();
        Self {
            enemy: moves.next().unwrap().parse().unwrap(),
            you: moves.next().unwrap().parse().unwrap(),
        }
    }

    fn from_part_2(s: &str) -> Self {
        let mut moves = s.split_whitespace();
        let move1 = moves.next().unwrap().parse::<Move>().unwrap();
        let what_to_do = moves.next().unwrap().parse::<WhatToDo>().unwrap();

        Self {
            enemy: move1,
            you: match (what_to_do, move1) {
                (Lose, Rock) => Scissors,
                (Lose, Paper) => Rock,
                (Lose, Scissors) => Paper,

                (Win, Rock) => Paper,
                (Win, Paper) => Scissors,
                (Win, Scissors) => Rock,

                (Draw, _) => move1,
            },
        }
    }

    fn calculate_value(&self) -> i32 {
        let shape_score = match self.you {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let win_score = match (self.you, self.enemy) {
            // Win
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
            // Draw
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            // Loss
            _ => 0,
        };

        shape_score + win_score
    }
}

fn generate_solution(rounds: Vec<Round>) -> i32 {
    rounds
        .iter()
        .fold(0, |acc, round| acc + round.calculate_value())
}

fn main() {
    let input = read_to_string("input/2022/day2.txt").unwrap();
    let trimmed = input.trim();

    let part_1_rounds = trimmed.lines().map(Round::from_part_1).collect::<Vec<_>>();

    println!("Part 1: {}", generate_solution(part_1_rounds));

    let part_2_rounds = trimmed.lines().map(Round::from_part_2).collect::<Vec<_>>();

    println!("Part 2: {}", generate_solution(part_2_rounds));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\n\
B X\n\
C Z\n\
    ";

    #[test]
    fn test_part1() {
        let rounds = INPUT.lines().map(Round::from_part_1).collect::<Vec<_>>();

        assert_eq!(generate_solution(rounds), 15);
    }

    #[test]
    fn test_part2() {
        let rounds = INPUT.lines().map(Round::from_part_2).collect::<Vec<_>>();

        assert_eq!(generate_solution(rounds), 12);
    }
}