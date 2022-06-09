use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn fuel_calc(mass: &u64, total: &mut u64) -> u64 {
    let fuel = match (mass / 3).cmp(&2) {
        cmp::Ordering::Less => 0,
        _ => (mass / 3) - 2,
    };

    *total += fuel;

    match fuel {
        0 => *total,
        _ => fuel_calc(&fuel, total),
    }
}

fn main() {
    let numbers: Vec<u64> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (u64 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();

    let total: u64 = numbers.iter().map(|n| fuel_calc(n, &mut 0)).sum();
    println!("Total: {}", total);
}
