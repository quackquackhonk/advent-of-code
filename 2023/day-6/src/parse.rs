use nom::bytes::complete::take_until;
use nom::character::complete::{digit1, space1};
use nom::multi::separated_list1;
use nom::IResult;

use crate::races::{Race, Races};

pub fn parse_races(input: &str) -> Races {
    let (_, races) = parse_races_inner(input).expect("This should parse!");
    races
}

pub fn parse_races_inner(input: &str) -> IResult<&str, Races> {
    let mut lines = input.split('\n');
    let time_line = lines.next().expect("We have at least 2 lines");
    let dist_line = lines.next().expect("We have at least 2 lines");

    let (time_line, _) = take_until(" ")(time_line)?;
    let (dist_line, _) = take_until(" ")(dist_line)?;
    let (_, times) = separated_list1(space1, digit1)(time_line.trim())?;
    let (_, dists) = separated_list1(space1, digit1)(dist_line.trim())?;

    let races = times
        .iter()
        .zip(dists)
        .map(|(time_str, dist_str)| {
            let time: usize = time_str.parse().expect("This is gonna be a number");
            let dist: usize = dist_str.parse().expect("This is gonna be a number");
            Race::new(time, dist)
        }).collect::<Races>();

    Ok(("", races))
}
