use std::collections::HashMap;

use crate::parse::parse_scratchcard;

pub fn process(input: &str) -> anyhow::Result<usize> {
    let total_cards_map = input.lines().filter_map(parse_scratchcard).fold(
        HashMap::<usize, usize>::new(),
        |mut tcm, sc| {
            let num_matches = sc.num_matches();
            // we add one to account for the current card
            let current_sc_count = match tcm.get(&sc.id) {
                Some(c) => c + 1,
                None => 1,
            };

            if num_matches == 0 {
                tcm.insert(sc.id, current_sc_count);
                return tcm;
            }

            let first_card_to_copy = sc.id + 1;
            let last_card_to_copy = sc.id + num_matches;
            for to_copy in first_card_to_copy..=last_card_to_copy {
                let to_add = current_sc_count;
                match tcm.get(&to_copy) {
                    Some(v) => tcm.insert(to_copy, v + to_add),
                    None => tcm.insert(to_copy, to_add),
                };
            }

            tcm.insert(sc.id, current_sc_count);

            tcm
        },
    );

    let output = total_cards_map.iter().fold(0, |acc, (key, val)| acc + val);

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::part2::process;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, process(input)?);
        Ok(())
    }
}
