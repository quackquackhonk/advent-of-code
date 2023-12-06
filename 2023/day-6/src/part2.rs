use crate::parse::parse_single_race;
use crate::races::Race;

pub fn process(input: &str) -> anyhow::Result<usize> {
    let race: Race = parse_single_race(input);
    Ok(race.ways_to_win())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
