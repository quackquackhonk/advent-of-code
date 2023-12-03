pub fn process(_input: &str) -> anyhow::Result<usize> {
    todo!("day-3 part2 unimplemented!");
}

#[cfg(test)]
mod tests {
    use crate::part2::process;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.. ";

        assert_eq!(467835, process(input).expect("Should give us a number"));
        Ok(())
    }
}
