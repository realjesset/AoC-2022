pub mod game;

use game::Game;

pub fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(input: &[Game]) -> u32 {
    // count the total score of all games
    let total_score = input
        .iter()
        .map(|game| game.play(|_, guide| guide))
        .sum::<u32>();

    println!("Part one: {}", total_score);

    total_score
}

pub fn part_two() {
    unimplemented!()
}
