use std::env;
use std::fs::read_to_string;
use std::cmp::{min, max};

#[derive(Debug, Copy, Clone)]
struct IdRange {
    begin: u64,
    end: u64,
}

impl IdRange {
    fn overlaps(&self, other: &IdRange) -> bool
    {
        let mut overlap: bool = false;

        if ( other.begin >= self.begin && other.begin <= self.end ) ||
           ( other.end >= self.begin && other.end <= self.end )
        {
            overlap = true;
        }
        else if other.begin < self.begin && other.end > self.end
        {
            overlap = true;
        }

        overlap
    }

    fn combine(&mut self, other: &IdRange)
    {
        self.begin = min(self.begin, other.begin);
        self.end = max(self.end, other.end);
    }
    fn total(&self) -> u64
    {
        self.end - self.begin + 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    let mut ranges: Vec<IdRange> = lines.split("\n\n")
                                        .nth(0).unwrap().split('\n')
                                        .map(|x| 
                                              IdRange {
                                                  begin: x.split('-').nth(0).unwrap().parse::<u64>().unwrap(),
                                                  end:   x.split('-').nth(1).unwrap().parse::<u64>().unwrap()
                                              } )
                                        .collect();
    let ids: Vec<u64> = lines.split("\n\n").nth(1).unwrap().split('\n').filter(|x|!x.is_empty())
                             .map(|x| x.parse::<u64>().unwrap())
                             .collect();


    // Part 1
    let mut fresh_ids = 0;
    for id in ids.iter()
    {
        for range in ranges.iter()
        {
            if (range.begin..=range.end).contains(id)
            {
                fresh_ids +=1;
                break;
            }
        }
    }
    println!("fresh_ids(part1)={fresh_ids}");

    // Part 2
    let mut idx = 0;
    loop
    {
        if idx == ranges.len()-1 { break; }

        let mut overlap = false;
        let mut overlap_idx: Vec<usize> = Vec::new();

        for i in idx+1..ranges.len()
        {
            if ranges[idx].overlaps(&ranges[i])
            {
                //println!("{idx} overlaps with {i}, combining...");
                let overlap_range = ranges[i].clone();
                overlap_idx.push(i);
                ranges[idx].combine(&overlap_range);
                overlap = true;
            }
        }

        let mut removed = 0;
        for oidx in overlap_idx
        {
            ranges.remove(oidx-removed);
            removed += 1;
        }

        if !overlap { idx += 1; }
    }

    let mut fresh_ids = 0;
    for range in ranges
    {
        fresh_ids += range.total();
    }
    println!("fresh_ids(part2)={fresh_ids}");
}