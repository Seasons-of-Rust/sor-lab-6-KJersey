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

fn run(noun: &u64, verb: &u64, numbers: &[u64]) -> u64 {
    let mut memory = numbers.to_vec();
    memory[1] = *noun;
    memory[2] = *verb;

    for chunk in memory.to_vec().chunks(4) {
        let operator = decode(&chunk[0]);

        if operator.is_none() {
            break;
        }

        let operand1 = memory[chunk[1] as usize];
        let operand2 = memory[chunk[2] as usize];

        memory[chunk[3] as usize] = operator.unwrap()(&operand1, &operand2);
    }

    memory[0]
}

fn main() {
    let mut file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut line = String::new();
    let _ = file.read_line(&mut line);

    let numbers: Vec<u64> = line
        .split(',')
        .map(|n| n.parse().expect("Cannot convert to int"))
        .collect();

    'noun: for noun in 0 .. 99 {
        '_verb: for verb in 0 .. 99 {
            if run(&noun, &verb, &numbers) == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'noun;
            }
        }
    }
}
