pub fn process(input: &str) -> anyhow::Result<usize> {
    todo!("day-7 part1 unimplemented!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
