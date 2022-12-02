use std::str::FromStr;

/**
 * RULES:
 *
 * (win by shape)
 * WIN with Rock -> 1
 * WIN with Paper -> 2
 * WIN with Scissors -> 3
 *
 * (game outcomes)
 * Lose -> 0
 * Draw -> 3
 * Win -> 6
 *
 * score => (win by shape) + (game outcomes)
 */

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub enum Outcomes {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Game {
    guide: Shape,
    opponent: Shape,
}

impl Game {
    pub fn new(opponent: Shape, guide: Shape) -> Self {
        Game { guide, opponent }
    }

    pub fn play<F>(&self, mut guess: F) -> u32
    where
        F: FnMut(Shape, Shape) -> Shape,
    {
        // println!(
        //     "{:?} vs {:?} -> {:?}",
        //     self.guide,
        //     self.opponent,
        //     guess(self.opponent, self.guide)
        // );

        let guess = guess(self.opponent, self.guide);

        let score = match (guess, self.opponent) {
            (Shape::Rock, Shape::Rock) => Outcomes::Draw as u32 + Shape::Rock as u32,
            (Shape::Rock, Shape::Paper) => Outcomes::Lose as u32 + Shape::Rock as u32,
            (Shape::Rock, Shape::Scissors) => Outcomes::Win as u32 + Shape::Rock as u32,
            (Shape::Paper, Shape::Rock) => Outcomes::Win as u32 + Shape::Paper as u32,
            (Shape::Paper, Shape::Paper) => Outcomes::Draw as u32 + Shape::Paper as u32,
            (Shape::Paper, Shape::Scissors) => Outcomes::Lose as u32 + Shape::Paper as u32,
            (Shape::Scissors, Shape::Rock) => Outcomes::Lose as u32 + Shape::Scissors as u32,
            (Shape::Scissors, Shape::Paper) => Outcomes::Win as u32 + Shape::Scissors as u32,
            (Shape::Scissors, Shape::Scissors) => Outcomes::Draw as u32 + Shape::Scissors as u32,
        };

        // println!("Score: {}", score);
        score
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let guide = split.next().unwrap().parse()?;
        let opponent = split.next().unwrap().parse()?;

        Ok(Game::new(guide, opponent))
    }
}
