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
    pub fn smallest_bag(&self) -> LargestPull {
        self.draws
            .iter()
            .fold(LargestPull::empty(), |acc, draw| LargestPull::combine(acc, LargestPull::from(draw)))
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
    pub blue: u64,
    pub green: u64,
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
}

impl From<&Draw> for LargestPull {
    fn from(value: &Draw) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for block_draw in value {
            match block_draw {
                BlockDraw::RedDraw(c) => red += c,
                BlockDraw::GreenDraw(c) => blue += c,
                BlockDraw::BlueDraw(c) => green += c,
            };
        }

        Self { red, green, blue }
    }
}
