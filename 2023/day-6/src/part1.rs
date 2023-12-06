use crate::parse::parse_races;

pub fn process(input: &str) -> anyhow::Result<usize> {
    let races: usize = parse_races(input).iter().map(|r| r.ways_to_win()).product();
    Ok(races)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, process(input)?);
        Ok(())
    }
}
