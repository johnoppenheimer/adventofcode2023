use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, info};
use regex::Regex;

#[derive(Eq, PartialEq, Clone)]
struct Game {
    id: usize,
    red: Vec<usize>,
    blue: Vec<usize>,
    green: Vec<usize>,
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Game {}: {:?}R {:?}G {:?}B",
            self.id, self.red, self.green, self.blue
        )
    }
}

impl Game {
    fn max_of(&self, color: &str) -> usize {
        match color {
            "red" => *self.red.iter().max().unwrap(),
            "green" => *self.green.iter().max().unwrap(),
            "blue" => *self.blue.iter().max().unwrap(),
            _ => 0,
        }
    }

    fn min_of(&self, color: &str) -> usize {
        match color {
            "red" => *self.red.iter().min().unwrap(),
            "green" => *self.green.iter().min().unwrap(),
            "blue" => *self.blue.iter().min().unwrap(),
            _ => 0,
        }
    }
}

/// Parse the input and return the whole game info
fn read_line_input(text: &str) -> Game {
    debug!("---- read line");
    let re = Regex::new(
        r"(Game (?P<game_id>\d+))|((?P<blue>\d+) blue)|((?P<green>\d+) green)|((?P<red>\d+) red)",
    )
    .unwrap();

    let mut game = Game {
        id: 0,
        red: vec![],
        blue: vec![],
        green: vec![],
    };

    let caps = re.captures_iter(text);
    debug!("{}", text);
    for cap in caps {
        if let Ok(game_id) = cap
            .name("game_id")
            .map_or("", |m| m.as_str())
            .parse::<usize>()
        {
            game.id = game_id;
        }

        if let Ok(blue) = cap.name("blue").map_or("", |m| m.as_str()).parse::<usize>() {
            game.blue.push(blue);
        }

        if let Ok(green) = cap
            .name("green")
            .map_or("", |m| m.as_str())
            .parse::<usize>()
        {
            game.green.push(green);
        }

        if let Ok(red) = cap.name("red").map_or("", |m| m.as_str()).parse::<usize>() {
            game.red.push(red);
        }
    }
    debug!("{:?}", game);

    return game;
}

fn resolve_part_1(games: Vec<Game>) -> usize {
    let found_games: Vec<Game> = games
        .into_iter()
        .filter(|game| {
            game.max_of("red") <= 12 && game.max_of("green") <= 13 && game.max_of("blue") <= 14
        })
        .collect();

    found_games.into_iter().map(|game| game.id).sum::<usize>()
}

fn resolve_part_2(games: Vec<Game>) -> usize {
    let powers: Vec<usize> = games
        .into_iter()
        .map(|game| game.max_of("blue") * game.max_of("red") * game.max_of("green"))
        .collect();

    debug!("powers: {:?}", powers);

    return powers.iter().sum();
}

pub fn run() {
    info!("--- DAY 2 ---");

    let filename = "./src/inputs/day_2.txt";
    let file = File::open(filename).expect("Couldn't open the file");
    let reader = BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            games.push(read_line_input(&l));
        }
    }

    let part_1 = resolve_part_1(games.clone());
    info!("Part 1: {}", part_1);

    let part_2 = resolve_part_2(games);
    info!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parsing() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = read_line_input(line);
        assert_eq!(
            game,
            Game {
                id: 1,
                blue: vec![3, 6],
                red: vec![4, 1],
                green: vec![2, 2],
            }
        );

        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = read_line_input(line);
        assert_eq!(
            game,
            Game {
                id: 2,
                blue: vec![1, 4, 1],
                red: vec![1],
                green: vec![2, 3, 1],
            }
        );
    }
}
