use std::fs;
use regex::Regex;

fn main() {
    let lines = grab_lines("input");
    let calibration_value: u32 = lines.iter().map(|l| parse_calibration_line(l.to_string())).sum();
    println!("{}", calibration_value);
}

fn grab_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_calibration_line(line: String) -> u32 {
    let word_digits = r"one|two|three|four|five|six|seven|eight|nine";
    let pattern = format!(r"\d|{}", word_digits);
    let rev_pat = format!(r"\d|{}", reverse(word_digits));
    let re = Regex::new(&pattern).unwrap();
    let rev_re = Regex::new(&rev_pat).unwrap();
    let rev_line = reverse(&line);

    let first = re.find(&line).unwrap().as_str();
    let last = rev_re.find(&rev_line).unwrap().as_str();

    (parse_match(first) * 10) + parse_match(&reverse(last))
}

fn reverse(s: &str) -> String {
    return s.chars().rev().collect::<String>()
}

fn parse_match(m: &str) -> u32 {
    match m {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse().unwrap(),
    }
}
