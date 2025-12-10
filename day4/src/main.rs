use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    let mut roll_grid: Vec<Vec<bool>> = Vec::new();
    
    for line in lines.split('\n').filter(|x|!x.is_empty())
    {
        let row: Vec<bool> = line.chars().map(|x| x == '@').collect();
        roll_grid.push(row); 
    }

    let mut removed_rolls = 0;
    loop
    {
        let mut accessible_rolls = 0;
        let mut accessible_positions: Vec<(usize, usize)> = Vec::new();
        for r in 0..roll_grid.len()
        {
            for c in 0..roll_grid[r].len()
            {
                if roll_grid[r][c]
                {
                    if count_adjacent_rolls(r, c, &roll_grid) < 4
                    {
                        accessible_positions.push((r,c));
                        accessible_rolls += 1;
                    }
                }
            }
        }
        for position in accessible_positions
        {
            roll_grid[position.0][position.1] = false;
        }
        removed_rolls += accessible_rolls;
        println!("removed {accessible_rolls} rolls.");
        if accessible_rolls == 0 { break; }
    }

    println!("removed a total of {removed_rolls} rolls");
}

fn count_adjacent_rolls(r: usize, c: usize, grid: &Vec<Vec<bool>> ) -> u8 
{
    let mut adj_rolls: u8 = 0;

    let mut min_r = 0;
    if r > 0 { min_r = r-1; }
    let mut max_r = r+1;
    if max_r > grid.len()-1 { max_r = grid.len()-1; }
    let mut min_c = 0;
    if c > 0 { min_c = c-1; }
    let mut max_c = c+1;
    if max_c > grid[r].len()-1 { max_c = grid[r].len()-1; }

    for ri in min_r..=max_r
    {
        for ci in min_c..=max_c
        {
            if grid[ri][ci] && (r,c) != (ri,ci)
            {
                adj_rolls += 1;
            }
        }
    }

    //print!("({r}, {c}) = {adj_rolls}");
    //if adj_rolls < 4 { print!("!!!");}
    //print!("\n");
    adj_rolls
}