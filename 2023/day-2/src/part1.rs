use crate::aoc_error::AocError;
use crate::parse::parse_game;

pub fn process(input: &str) -> anyhow::Result<u64, AocError> {
    let output: u64 = input.lines().filter_map(process_line).sum();

    Ok(output)
}

/// Process a line of input. This function will parse the line into a Game,
/// checks if the game is valid, returning the ID of the game if valid,
/// and None otherwise
pub fn process_line(line: &str) -> Option<u64> {
    let (_, game) = parse_game(line).ok()?;
    let largest = game.largest_pull();

    if largest.possible(12, 13, 14) {
        Some(game.id)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input)?, 8);
        Ok(())
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Some(1))]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Some(2))]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", None)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", None)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Some(5))]
    pub fn test_process_line(#[case] input: &str, #[case] expected: Option<u64>) {
        assert_eq!(expected, process_line(input));
    }

}
