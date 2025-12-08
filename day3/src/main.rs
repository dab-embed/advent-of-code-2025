use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    for line in lines.split('\n').filter(|x|!x.is_empty())
    {
        let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let joltage = &bank[..bank.len()].iter().max().unwrap();

        println!("joltage = {joltage:?}");
    }
}