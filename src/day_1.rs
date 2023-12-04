use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, info};
use regex::{Regex, RegexSet};

#[derive(Eq, PartialEq, Clone)]
struct TextPosition {
    text: String,
    start_index: usize,
    end_index: usize,
}

impl fmt::Debug for TextPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} - {})",
            self.text, self.start_index, self.end_index
        )
    }
}

fn text_to_digit(text: &str) -> &str {
    match text {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => text,
    }
}

/// Replace written numbers in the text by its digit version
/// i.e: two -> 2
/// # Arguments
/// * `text` - Text to parse
/// ```
/// let text = "twotwo6eightqcncghjvqfcv5";
/// let updatedText = extract_numbers(&text);
/// assert_eq!(updatedText, "2268qcncghjvqfcv5");
fn extract_numbers(text: &str) -> usize {
    debug!("----");
    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let set = RegexSet::new(patterns).unwrap();
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    // let re =
    //     Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();
    let mut positions: Vec<TextPosition> = set
        .matches(text)
        .into_iter()
        .map(|index| &regexes[index])
        .map(|re| re.find_iter(text))
        .flatten()
        .map(|m| TextPosition {
            text: text_to_digit(m.as_str()).to_string(),
            start_index: m.start(),
            end_index: m.end(),
        })
        .collect();

    debug!("{:?}", text);
    debug!("Positions: {:?}", positions);

    positions.sort_by(|a, b| a.start_index.cmp(&b.start_index));
    let first_pos = positions.first().cloned().unwrap();

    positions.sort_by(|a, b| b.end_index.cmp(&a.end_index));
    let last_pos = positions.first().clone().unwrap();

    if first_pos.eq(last_pos) {
        return format!("{}{}", first_pos.text, first_pos.text)
            .parse()
            .unwrap();
    }

    return format!("{}{}", first_pos.text, last_pos.text)
        .parse()
        .unwrap();
}

pub fn run() {
    info!("--- DAY 1 ----");

    let filename = "./src/inputs/day_1.txt";
    let file = File::open(filename).expect("Couldn't read file");
    let reader = BufReader::new(file);

    let mut numbers: Vec<usize> = vec![];

    for line in reader.lines() {
        if let Ok(l) = line {
            let res = extract_numbers(&l);
            debug!("{} -> {}", l, res);
            numbers.push(res);
        }
    }

    let sum = numbers.into_iter().fold(0, |acc, el| acc + el);
    info!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_replace_then_extract() {
        init();

        let res = extract_numbers("twotwo6eightqcncghjvqfcv5");
        assert_eq!(res, 25);

        let res = extract_numbers("fourclkthghllzlhrs31");
        assert_eq!(res, 41);

        let res = extract_numbers("4d");
        assert_eq!(res, 44);

        let res = extract_numbers("fivetwo9");
        assert_eq!(res, 59);

        let res = extract_numbers("k3");
        assert_eq!(res, 33);

        let res = extract_numbers("xtwone3four");
        assert_eq!(res, 24);

        let res = extract_numbers("oneightwo");
        assert_eq!(res, 12);

        let res = extract_numbers("oneightwo3");
        assert_eq!(res, 13);

        let res = extract_numbers("6512krnnxdxzprbtlgcfoneeightwohfl");
        assert_eq!(res, 62);

        let res = extract_numbers("prlhtzthtwo3mjrblrtrsfoneeight4fourtwo");
        assert_eq!(res, 22);
    }
}
