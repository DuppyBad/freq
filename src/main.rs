use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_frequency(input: &str) -> HashMap<char, i32> {
    let mut frequency = HashMap::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            *frequency.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    frequency
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut total_frequency = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let frequency = calculate_frequency(&line);
        for (c, f) in frequency {
            *total_frequency.entry(c).or_insert(0) += f;
        }
    }

    let mut sorted_frequency: Vec<(&char, &i32)> = total_frequency.iter().collect();
    sorted_frequency.sort_by_key(|&(c, _)| *c);

    for (c, f) in sorted_frequency {
        println!("{}: {}", c, f);
    }
}