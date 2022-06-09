use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers: Vec<u64> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();

    let total: u64 = numbers.iter().map(|n| ((n / 3) - 2)).sum();
    println!("Total: {}", total);
}
