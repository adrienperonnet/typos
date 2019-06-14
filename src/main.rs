extern crate clap;

use clap::{App, Arg};
use std::str::FromStr;

mod distance;

use crate::distance::PathFindingAlorithm;
use crate::distance::PathFindingAlorithm::{Astar, Dijkstra, Fringe, Idastar};
use core::borrow::Borrow;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let default_algorithm = format!("{}", Astar);
    let matches = App::new("typos")
        .version("1.0")
        .author("Adrien adrien@apapa.fr")
        .about("Find a shortest edit-path between two input words")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ALGORITHM")
                .short("a")
                .long("algorithm")
                .help("algorithm to use to compute shortest path")
                .possible_value(format!("{}", Astar).as_str())
                .possible_value(format!("{}", Idastar).as_str())
                .possible_value(format!("{}", Dijkstra).as_str())
                .possible_value(format!("{}", Fringe).as_str())
                .default_value(default_algorithm.as_str())
                .index(4),
        )
        .arg(
            Arg::with_name("START")
                .short("s")
                .long("start")
                .help("starting word")
                .case_insensitive(true)
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("END")
                .short("e")
                .long("end")
                .help("ending word")
                .case_insensitive(true)
                .required(true)
                .index(3),
        )
        .get_matches();

    //Safe unwrapping thanks to clap validation
    let filename = matches.value_of("INPUT").unwrap();
    let start = matches.value_of("START").unwrap().to_lowercase();
    let start = start.as_str();
    let stop = matches.value_of("END").unwrap().to_lowercase();
    let stop = stop.as_str();
    let algorithm = matches
        .value_of("ALGORITHM")
        .map(PathFindingAlorithm::from_str)
        .unwrap()
        .unwrap();

    println!(
        "Using input file: {} with {} algorithm to compute shortest path between {} and {}",
        filename, algorithm, start, stop
    );

    let mut words: Vec<String> = lines_from_file(filename)
        .unwrap()
        .iter()
        .map(|w| w.to_lowercase())
        .collect();
    words.insert(0, stop.to_string());

    let words: Vec<&str> = words.iter().map(AsRef::as_ref).collect();

    let word_count = words.len() + 1;

    println!("{} words loaded into memory", word_count);
    let start_time = Instant::now();
    let res = distance::find_shortest_path(start, stop, words.as_slice(), algorithm.borrow());
    let duration = start_time.elapsed();
    match res.map(|(p, d)| (p.join("->"), d)) {
        Some((words, cost)) => println!(
            "Shortest path found in {:?}: {} (achieved in {})",
            duration, words, cost
        ),
        None => println!("No path found, something went wrong ?"),
    }
}
