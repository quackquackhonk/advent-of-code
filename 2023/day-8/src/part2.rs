use crate::network::GhostMap;
use crate::parse::parse_map;

pub fn process(input: &str) -> anyhow::Result<i64> {
    let (_, map) = parse_map(input).expect("This should parse!");
    Ok(GhostMap::from(map).go_to())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
