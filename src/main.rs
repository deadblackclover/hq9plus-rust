use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Unknown file name!");
    }

    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut plus = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "H".to_string() {
            println!("Hello, world!");
        } else if line == "Q".to_string() {
            let contents =
                fs::read_to_string(filename).expect("Something went wrong reading the file");

            println!("{}", contents);
        } else if line == "9".to_string() {
            for i in (1..99).rev() {
                println!("{} bottles of beer on the wall,", i);
                println!("{} bottles of beer.", i);
                println!("Take one down, pass it around,");
                println!("{} bottles of beer on the wall.", i - 1);
            }
        } else if line == "+".to_string() {
            plus += 1;
            println!("+");
        }
    }

    println!("Accumulator value: {}", plus);
}
