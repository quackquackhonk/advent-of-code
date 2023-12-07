use nom::IResult;
use nom::bytes::complete::take;
use nom::character::complete::{newline, space1, digit1};
use nom::multi::{separated_list1, count};
use nom::sequence::separated_pair;

use crate::cards::{Hand, Card};



pub fn parse_hands(input: &str) -> Vec<Hand> {
    let (_, hands) = parse_hands_inner(input).expect("This should parse!");
    hands
}

pub fn parse_hands_inner(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(newline, parse_hand)(input)
}

pub fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(parse_cards, space1, digit1)(input)?;
    let bid: usize = bid.parse().expect("This is a number");

    Ok((input, Hand::new(cards, bid)))
}

pub fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, card_strs) = count(take(1_usize), 5)(input)?;
    let cards = card_strs.iter().map(|c| Card::from(*c)).collect();
    Ok((input, cards))
}
