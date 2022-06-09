use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn add(operand1: &u64, operand2: &u64) -> u64 {
    operand1 + operand2
}

fn mul(operand1: &u64, operand2: &u64) -> u64 {
    operand1 * operand2
}

fn decode(opcode: &u64) -> Option::<fn(&u64, &u64) -> u64> {
    match opcode {
        1 => Some(add),
        2 => Some(mul),
        99 => None,
        _ => panic!("Invalid opcode"),
    }
}

fn main() {
    let mut file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut line = String::new();
    let _ = file.read_line(&mut line);

    let mut numbers: Vec<u64> = line
        .split(',')
        .map(|n| n.parse().expect("Cannot convert to int"))
        .collect();

    numbers[1] = 12;
    numbers[2] = 2;

    for chunk in numbers.to_vec().chunks(4) {
        let operator = decode(&chunk[0]);

        if operator.is_none() {
            break;
        }

        let operand1 = numbers[chunk[1] as usize];
        let operand2 = numbers[chunk[2] as usize];

        numbers[chunk[3] as usize] = operator.unwrap()(&operand1, &operand2);
    }

    println!("{}", numbers[0])
}
