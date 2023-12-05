use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::scratchcard::ScratchCard;

pub fn parse_scratchcard(input: &str) -> Option<ScratchCard> {
    let (_, card) = parse_scratchcard_inner(input).ok()?;
    Some(card)
}

pub fn parse_scratchcard_inner(input: &str) -> IResult<&str, ScratchCard> {
    let (input, _) = tag("Card")(input)?;
    let (input, id_str) = digit1(input.trim_start())?;
    let (input, _) = tag(":")(input)?;
    let input = input.trim();

    let (input, (winning, revealed)) = separated_pair(
        parse_scratchcard_numbers,
        tag(" | "),
        parse_scratchcard_numbers,
    )(input)?;

    let id: usize = id_str.parse().expect("Input should be well formed");
    Ok((input, ScratchCard::new(id, winning, revealed)))
}

pub fn parse_scratchcard_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, num_strs) = separated_list1(space1, digit1)(input.trim())?;
    let nums = num_strs
        .iter()
        .map(|s| s.parse::<usize>().expect("Input should be well formed"))
        .collect();
    Ok((input, nums))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::scratchcard::ScratchCard;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", ScratchCard::new(1, vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]))]
    pub fn test_parse_scratchcard(#[case] input: &str, #[case] expected: ScratchCard) {
        assert_eq!(expected, parse_scratchcard(input).unwrap());
    }
}
