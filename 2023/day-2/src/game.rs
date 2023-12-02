/// A Game contains information about the choosing game played
#[derive(Debug, PartialEq)]
pub struct Game {
    /// The ID of the game
    pub id: u64,
    /// A list of blocks drawn from the bag during the game
    pub draws: Vec<Draw>,
}

pub type Draw = Vec<BlockDraw>;

/// The possible blocks to draw from the bag
#[derive(Debug, PartialEq)]
pub enum BlockDraw {
    RedDraw(u64),
    BlueDraw(u64),
    GreenDraw(u64),
}

/// A structure representing the Bag that the Game was played with.
#[derive(Debug, PartialEq)]
pub struct Bag {
    pub red: u64,
    pub blue: u64,
    pub green: u64,
}

impl Game {
    pub fn new(id: u64, draws: Vec<Draw>) -> Self {
        Self { id, draws }
    }
    /// Scans the draws that occured in the game, and returns the smallest possible bag
    /// that the game could have been played with.
    pub fn smallest_bag() -> Bag {
        unimplemented!()
    }
}
