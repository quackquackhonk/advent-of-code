use itertools::Itertools;

use crate::parse::parse_readings;

pub fn process(input: &str) -> anyhow::Result<isize> {
    let vals = parse_readings(input).iter().map(|r| r.extrapolate()).collect_vec();

    Ok(vals.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
