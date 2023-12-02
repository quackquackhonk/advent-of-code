use crate::aoc_error::AocError;

pub fn process(input: &str) -> anyhow::Result<u64, AocError> {
    let output: u64 = input.lines().filter_map(process_line).sum();

    Ok(output)
}

/// Process a line of input. This function will parse the line into a Game,
/// checks if the game is valid, returning the ID of the game if valid,
/// and None otherwise
pub fn process_line(line: &str) -> Option<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let test_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(test_str)?, 8);
        Ok(())
    }
}
