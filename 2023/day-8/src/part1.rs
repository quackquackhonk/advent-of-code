use crate::parse::parse_map;

pub fn process(input: &str) -> anyhow::Result<usize> {
    let (_, map) = parse_map(input).expect("This should parse!");
    Ok(map.go_to("ZZZ"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
