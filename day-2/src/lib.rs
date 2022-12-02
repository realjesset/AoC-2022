pub mod game;

use game::{Game, Shape};

pub fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(input: &[Game]) -> u32 {
    // count the total score of all games
    let total_score = input
        .iter()
        .map(|game| game.play(|_, guide| guide))
        .sum::<u32>();

    total_score
}

pub fn part_two(input: &[Game]) -> u32 {
    // count the total score of all games
    let total_score = input
        .iter()
        .map(|game| {
            game.play(|opponent, guide| {
                // print!("{:?} vs {:?}", opponent, guide);
                let ans = match (opponent, guide) {
                    (Shape::Rock, Shape::Rock) => Shape::Scissors, // lose,
                    (Shape::Rock, Shape::Paper) => Shape::Rock,    // draw
                    (Shape::Rock, Shape::Scissors) => Shape::Paper, // win
                    (Shape::Paper, Shape::Rock) => Shape::Rock,    // lose
                    (Shape::Paper, Shape::Paper) => Shape::Paper,  // draw
                    (Shape::Paper, Shape::Scissors) => Shape::Scissors, // win
                    (Shape::Scissors, Shape::Rock) => Shape::Paper, // lose
                    (Shape::Scissors, Shape::Paper) => Shape::Scissors, // draw
                    (Shape::Scissors, Shape::Scissors) => Shape::Rock, // win
                };
                // println!(" -> {:?}", ans);
                ans
            })
        })
        .sum::<u32>();

    total_score
}
