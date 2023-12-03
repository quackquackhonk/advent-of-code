use std::collections::HashMap;

use itertools::Itertools;
use regex::{Match, Regex};

use crate::gear::Gear;

pub type Index = (usize, usize);

pub fn process(input: &str) -> anyhow::Result<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let num_map = create_number_map(&lines);

    let gear_centers = create_gear_centers(&lines);

    let o: usize = create_gears_list(gear_centers, num_map).iter().filter_map(|gear| {
        if gear.valid() {
            Some(gear.gear_ratio())
        }
        else {
            None
        }
    }).sum();

    Ok(o)
}

pub fn create_gears_list(centers: Vec<Index>, num_map: HashMap<Index, usize>) -> Vec<Gear> {
    centers.iter().map(|center_idx| Gear::from_center_and_nums(center_idx, &num_map)).collect()
}

pub fn create_gear_centers(lines: &[&str]) -> Vec<Index> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(char_idx, char)| {
                    if char == '*' {
                        Some((line_idx, char_idx))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

pub fn create_number_map(lines: &[&str]) -> HashMap<Index, usize> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            let re = Regex::new(r"\d+").unwrap();
            re.find_iter(line)
                .map(|m| (line_idx, m))
                .collect::<Vec<(usize, Match)>>()
        })
        .fold(HashMap::new(), |mut acc, (line_idx, m)| {
            let number = m
                .as_str()
                .parse::<usize>()
                .expect("This will always be a number");
            for digit_idx in m.range() {
                acc.insert((line_idx, digit_idx), number);
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::part2::{process, create_number_map, create_gear_centers, create_gears_list};

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

        let lines: Vec<&str> = input.lines().collect();
        let num_map = create_number_map(&lines);
        let centers = create_gear_centers(&lines);
        dbg!(create_gears_list(centers, num_map));

        assert_eq!(467835, process(input).expect("Should give us a number"));
        Ok(())
    }
}
