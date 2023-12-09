use crate::readings::Reading;

pub fn parse_readings(input: &str) -> Vec<Reading> {
    input
        .lines()
        .map(|l| {
            let sequence = l
                .split(' ')
                .map(|n| n.parse::<isize>().expect("number"))
                .collect();
            Reading(sequence)
        })
        .collect()
}
