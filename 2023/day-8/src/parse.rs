use std::collections::HashMap;

use nom::IResult;
use nom::bytes::complete::{take_until, take, tag};
use nom::character::complete::newline;
use nom::multi::separated_list1;

use crate::network::{Branch, Map, Direction};

pub fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, directions) = parse_directions(input)?;
    let input = input.trim();
    let (input, edges) = parse_edges(&input)?;
    Ok((input, Map::new(directions, "AAA".to_string(), edges)))
}

pub fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, dirs) = take_until("\n\n")(input)?;
    let directions = dirs.chars().map(Direction::from).collect();
    Ok((input, directions))
}

pub fn parse_edges(input: &str) -> IResult<&str, HashMap<String, Branch>> {
    let (input, edges) = separated_list1(newline, parse_edge)(input)?;
    let mut edge_map: HashMap<String, Branch> = HashMap::new();
    for (node, branch) in edges {
        edge_map.insert(node, branch);
    }

    Ok((input, edge_map))
}

pub fn parse_edge(input: &str) -> IResult<&str, (String, Branch)> {
    let (input, node) = take(3_usize)(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take(3_usize)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take(3_usize)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (node.to_string(), Branch::new(left.to_string(), right.to_string()))))

}
