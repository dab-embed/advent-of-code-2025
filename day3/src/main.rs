use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    // Part 1
    let mut total_joltage: u32 = 0;
    
    for line in lines.split('\n').filter(|x|!x.is_empty())
    {
        let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut joltage: u32;
        let mut max_battery = 0;
        let mut max_index: usize = 0;
        for (index, battery) in bank[..bank.len()-1].iter().enumerate() 
        {
            if *battery > max_battery
            {
                max_battery = *battery;
                max_index = index;
            }
        }
        joltage = max_battery;

        max_battery = 0;
        for battery in bank[max_index+1..].iter() 
        {
            if *battery > max_battery 
            {
                max_battery = *battery;
            }
        }
        let joltage = 10*joltage + max_battery;
        total_joltage += joltage;
    }
    println!("total_joltage(part1)={total_joltage}");

    // Part 2
    const JOLTAGE_LEN: usize = 12;
    let mut total_joltage: u64 = 0;
    
    for line in lines.split('\n').filter(|x|!x.is_empty())
    {
        let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut search_win = bank.len()-JOLTAGE_LEN + 1;
        let mut search_start = 0;
        let mut joltage: u64 = 0;
        while search_start + search_win <= bank.len()
        {
            let mut max_battery = 0;
            let mut max_index = 0;
            for (index, battery) in bank[search_start..search_start+search_win].iter().enumerate()
            {
                if *battery > max_battery
                {
                    max_battery = *battery;
                    max_index = index;
                }
            }
            joltage *= 10;
            joltage += max_battery as u64;

            search_win -= max_index;
            search_start += max_index + 1;
        }
        total_joltage += joltage;
    }
    println!("total_joltage(part2)={total_joltage}");
}