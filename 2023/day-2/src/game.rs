use std::cmp::max;

/// A Game contains information about the choosing game played
#[derive(Debug, PartialEq)]
pub struct Game {
    /// The ID of the game
    pub id: u64,
    /// A list of blocks drawn from the bag during the game
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn new(id: u64, draws: Vec<Draw>) -> Self {
        Self { id, draws }
    }
    /// Scans the draws that occured in the game, and returns the smallest possible bag
    /// that the game could have been played with.
    pub fn largest_pull(&self) -> LargestPull {
        self.draws.iter().fold(LargestPull::empty(), |acc, draw| {
            LargestPull::combine(acc, LargestPull::from(draw))
        })
    }
}

pub type Draw = Vec<BlockDraw>;

/// The possible blocks to draw from the bag
#[derive(Debug, PartialEq)]
pub enum BlockDraw {
    RedDraw(u64),
    GreenDraw(u64),
    BlueDraw(u64),
}

/// A structure representing the largest simultaneous pull for each color of cube
#[derive(Debug, PartialEq)]
pub struct LargestPull {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl LargestPull {
    pub fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }

    pub fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn possible(&self, red: u64, green: u64, blue: u64) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }

    pub fn combine(left: LargestPull, right: LargestPull) -> LargestPull {
        Self::new(
            max(left.red, right.red),
            max(left.green, right.green),
            max(left.blue, right.blue),
        )
    }

    pub fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl From<&Draw> for LargestPull {
    fn from(value: &Draw) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for block_draw in value {
            match block_draw {
                BlockDraw::RedDraw(c) => red += c,
                BlockDraw::GreenDraw(c) => green += c,
                BlockDraw::BlueDraw(c) => blue += c,
            };
        }

        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::parse::parse_game;
    use super::*;

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        LargestPull::new(4, 2, 6)
    )]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        LargestPull::new(1, 3, 4)
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        LargestPull::new(20, 13, 6)
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        LargestPull::new(14, 3, 15)
    )]
    #[case(
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        LargestPull::new(6, 3, 2)
    )]
    fn test_largest_pull(#[case] input: &str, #[case] expected: LargestPull) -> anyhow::Result<()> {
        let (_, game) = parse_game(input).unwrap();
        assert_eq!(expected, game.largest_pull());
        Ok(())
    }
}
