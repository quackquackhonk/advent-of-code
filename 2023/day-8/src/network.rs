use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Map {
    pub directions: Vec<Direction>,
    pub edges: HashMap<String, Branch>,
    pub start: String
}

impl Map {
    pub fn new(directions: Vec<Direction>, start: String, edges: HashMap<String, Branch>) -> Self {
        Self { directions, edges, start }
    }

    pub fn go_to(&self, dest: &str) -> usize {
        let mut curr = &self.start;
        let mut count = 0;
        for dir in self.directions.iter().cycle() {
            let Some(branch) = self.edges.get(curr) else {
                unreachable!("All nodes have entries!");
            };
            curr = branch.go(dir);
            count += 1;

            if curr == dest {
                break
            }
        }
        count
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Branch {
    pub left: String,
    pub right: String,
}

impl Branch {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }

    pub fn go(&self, dir: &Direction) -> &String {
        match dir {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("Got {}", value)
        }
    }
}
