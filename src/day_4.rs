use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, info};
use regex::Regex;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Card {
    id: usize,
    winnings: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let mut points = 0;
        for number in &self.numbers {
            if self.winnings.contains(number) {
                if points == 0 {
                    points = 1
                } else {
                    points *= 2;
                }
            }
        }
        return points;
    }

    fn winning_copies(&self) -> Vec<usize> {
        return self
            .numbers
            .iter()
            .filter(|n| self.winnings.contains(n))
            .enumerate()
            .map(|(i, _n)| i + 1 + self.id)
            .collect();
    }
}

fn parse_card(text: &str) -> Card {
    let re = Regex::new(r"(Card +(?P<id>(\d+)): )").unwrap();
    let id = re.captures(text).unwrap().name("id").unwrap().as_str();

    let new_text = text.replace(re.captures(text).unwrap().get(0).unwrap().as_str(), "");

    let mut card = Card {
        id: id.parse::<usize>().unwrap(),
        winnings: vec![],
        numbers: vec![],
    };

    let blocks: Vec<String> = new_text.split("|").map(|s| s.trim().to_string()).collect();
    if let Some(winnings) = blocks.first() {
        card.winnings = winnings
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
    }

    if let Some(numbers) = blocks.last() {
        card.numbers = numbers
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
    }

    return card;
}

fn run_part_2(cards: Vec<Card>) {
    let mut copies: HashMap<usize, usize> = HashMap::new();

    for card in cards.clone() {
        debug!("------------------");
        debug!("copies: {:?}", copies);
        debug!("Card: {:?}", card.id);

        let winings_card = card.winning_copies();
        debug!("Winnings: {:?}", winings_card);

        if let Some(card_copies) = copies.get(&card.id) {
            for _n in 1..=*card_copies {
                for card_copy_id in winings_card.clone() {
                    copies
                        .entry(card_copy_id)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }

        copies.entry(card.id).and_modify(|c| *c += 1).or_insert(1);

        for card_copy_id in winings_card.clone() {
            copies
                .entry(card_copy_id)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    let total = copies.iter().map(|(_k, v)| v).sum::<usize>();
    info!("Part 2: {}", total);
}

pub fn run() {
    info!("--- DAY 4 ----");

    let filename = "./src/inputs/day_4.txt";
    let file = File::open(filename).expect("Couldn't read file");
    let reader = BufReader::new(file);

    let mut cards: Vec<Card> = vec![];

    for line in reader.lines() {
        if let Ok(l) = line {
            cards.push(parse_card(&l))
        }
    }

    let points = cards.iter().map(|c| c.points()).sum::<usize>();
    info!("Part 1: {}", points);

    run_part_2(cards);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_card() {
        let text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(text);

        assert_eq!(
            card,
            Card {
                id: 1,
                winnings: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );

        let text = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = parse_card(text);

        assert_eq!(
            card,
            Card {
                id: 2,
                winnings: vec![13, 32, 20, 16, 61],
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            }
        )
    }

    #[test]
    fn test_card_points() {
        let text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(text);
        assert_eq!(card.points(), 8);

        let text = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = parse_card(text);
        assert_eq!(card.points(), 2);

        let text = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let card = parse_card(text);
        assert_eq!(card.points(), 0);
    }

    #[test]
    fn test_winning_copies() {
        let text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(text);
        assert_eq!(card.winning_copies(), vec![2, 3, 4, 5]);

        let text = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = parse_card(text);
        assert_eq!(card.winning_copies(), vec![3, 4]);
    }
}
