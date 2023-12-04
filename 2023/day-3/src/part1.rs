
use regex::Regex;

pub fn process(input: &str) -> anyhow::Result<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let mask = make_symbol_mask(&lines)?;

    let re = Regex::new(r"[0-9]+")?;
    let output = lines.iter().enumerate().fold(0, |acc, (idx, line)| {
        let matches = re.find_iter(line);
        let sum_of_line: usize = matches
            .filter_map(|m| convert_if_part_number(&mask, m, idx))
            .sum();
        acc + sum_of_line
    });

    Ok(output)
}

pub fn convert_if_part_number(
    mask: &[Vec<bool>],
    m: regex::Match,
    line_idx: usize,
) -> Option<usize> {
    let convert = m.range().fold(false, |acc, digit_idx| {
        if !acc {
            acc || digit_touching_symbol(mask, digit_idx, line_idx)
        } else {
            acc
        }
    });

    if convert {
        Some(m.as_str().parse::<usize>().expect("This is a number"))
    } else {
        None
    }
}

pub fn digit_touching_symbol(mask: &[Vec<bool>], digit_idx: usize, line_idx: usize) -> bool {
    let max_line_idx = mask.len() - 1;
    let max_digit_idx = mask[0].len() - 1;
    let mut touching = false;

    // check above
    if line_idx != 0 {
        touching = touching || mask[line_idx - 1][digit_idx];

        if digit_idx != 0 {
            touching = touching || mask[line_idx - 1][digit_idx - 1];
        }
        if digit_idx != max_digit_idx {
            touching = touching || mask[line_idx - 1][digit_idx + 1];
        }
    } 

    // check below
    if line_idx != max_line_idx {
        touching = touching || mask[line_idx + 1][digit_idx];

        if digit_idx != 0 {
            touching = touching || mask[line_idx + 1][digit_idx - 1];
            // also check directly left
            touching = touching || mask[line_idx][digit_idx - 1];
        }
        if digit_idx != max_digit_idx {
            touching = touching || mask[line_idx + 1][digit_idx + 1];
            // also check directly right
            touching = touching || mask[line_idx][digit_idx + 1];
        }
    }

    touching
}

pub fn make_symbol_mask(lines: &[&str]) -> anyhow::Result<Vec<Vec<bool>>> {
    let mask: Vec<Vec<bool>> = lines
        .iter()
        .map(|l| l.chars().map(is_symbol).collect())
        .collect();

    Ok(mask)
}

pub fn is_symbol(c: char) -> bool {
    let symbols = [
        '!', '@', '#', '$', '%', '^', '&', '_', '/', '+', '*', '-', '=',
    ];

    symbols.contains(&c)
}

#[cfg(test)]
mod tests {
    use crate::part1::process;

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

        assert_eq!(4361, process(input).expect("Should give us a number"));
        Ok(())
    }
}
