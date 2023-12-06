use human_time::ToHumanTimeString;
use log::info;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct MapLine {
    destination: usize,
    source: usize,
    source_max: usize,
}

impl MapLine {
    fn is_in_range(&self, value: usize) -> bool {
        value >= self.source && value < self.source_max
    }
}

fn find_source_to_destination(value: usize, maps: &Vec<MapLine>) -> usize {
    maps.iter()
        .find_map(|map_line| {
            if map_line.is_in_range(value) {
                return Some(value - map_line.source + map_line.destination);
            }
            None
        })
        .unwrap_or(value)
}

fn parse(reader: BufReader<File>) -> (Vec<usize>, Vec<Vec<MapLine>>) {
    let mut seeds: Vec<usize> = vec![];

    let mut maps: Vec<Vec<MapLine>> = Vec::with_capacity(7);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        if line.starts_with("seeds:") {
            seeds = line
                .replace("seeds: ", "")
                .split(" ")
                .filter_map(|s| s.parse().ok())
                .collect();
        }

        if line.contains("map:") {
            maps.push(vec![]);
        } else if line == "" {
            //
        } else {
            let values: Vec<usize> = line.split(" ").filter_map(|s| s.parse().ok()).collect();
            let map_line = MapLine {
                destination: values[0],
                source: values[1],
                source_max: values[1] + values[2],
            };

            if let Some(mp) = maps.last_mut() {
                mp.push(map_line);
            }
        }
    }

    (seeds, maps)
}

pub fn run() {
    info!("--- DAY 5 ----");

    let now = std::time::Instant::now();

    let filename = "./src/inputs/day_5.txt";
    let file = File::open(filename).expect("Couldn't read file");
    let reader = BufReader::new(file);
    let (init_seeds, maps) = parse(reader);

    let seeds = init_seeds
        .into_par_iter()
        .chunks(2)
        .flat_map(|r| (r[0]..r[0] + r[1]).collect::<Vec<usize>>())
        .map(|seed| {
            let mut sed = seed.clone();

            for map in &maps {
                let destination = find_source_to_destination(sed, &map);
                sed = destination;
            }

            sed
        })
        .min();

    info!("lowest: {:?}", seeds.unwrap());

    info!("executed in {:?}", now.elapsed().to_human_time_string());
}
