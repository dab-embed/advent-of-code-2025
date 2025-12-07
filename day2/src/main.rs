use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct IdRange {
    begin: u64,
    end: u64,
}

impl IdRange {
    fn bad_ids_part1(&self) -> Vec<u64> {
        let mut ids: Vec<u64> = Vec::new();

        for i in self.begin..=self.end
        {
            let id: String = i.to_string();            
            if id.len() % 2 == 0
            {
               let (a, b) = id.split_at(id.len()/2);
               if a == b 
               {
                    //println!("Found bad id = {id}");
                    ids.push(i);
               } 
            }
        }
        ids
    }
    fn bad_ids_part2(&self) -> Vec<u64> {
        let mut ids: Vec<u64> = Vec::new();

        for i in self.begin..=self.end
        {
            let id: String = i.to_string();            

            for j in 1..id.len()
            {
                // Split String into vector of string of equal size of j
                let values = id.chars()
                               .collect::<Vec<char>>()
                               .chunks(j)
                               .map(|chunk|chunk.iter().collect::<String>())
                               .collect::<Vec<String>>();

                let first = values.iter().nth(0).unwrap();
                if values.iter().all(|x|x == first)
                {
                    //println!("Found bad id = {id}");
                    ids.push(i);
                    break;
                }
            }
        }
        ids
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    for line in lines.split('\n').filter(|x: &&str|!x.is_empty()) {
        let mut id_ranges: Vec<IdRange> = Vec::new();
        for id_range in line.split(',') {
            id_ranges.push(IdRange {
                begin: id_range.split('-').nth(0).unwrap().parse().unwrap(),
                end:   id_range.split('-').nth(1).unwrap().parse().unwrap()
            });
        }

        let mut bad_ids_part1: Vec<u64> = Vec::new();
        let mut bad_ids_part2: Vec<u64> = Vec::new();
        for range in id_ranges.iter() {
            bad_ids_part1.extend(range.bad_ids_part1());
            bad_ids_part2.extend(range.bad_ids_part2());
        }

        let total1: u64 = bad_ids_part1.iter().sum();
        let total2: u64 = bad_ids_part2.iter().sum();
        println!("Bad IDs total (part1) = {total1}, (part2) = {total2}");

    }
}
