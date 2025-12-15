use std::env;
use std::fs::read_to_string;

fn print_grid(grid: &Vec<Vec<char>>)
{
    for _ in 0..grid[0].len() { print!("#");}
    println!();
    for row in grid
    {
        for val in row
        {
            print!("{val}");
        }
        println!();
    }
    for _ in 0..grid[0].len() { print!("#");}
    println!();
}

fn place_beam(grid: &mut Vec<Vec<char>>, pos: (usize, usize))
{
    if grid[pos.0][pos.1] == '.'
    {
        for i in pos.0..grid.len()
        {
             if grid[i][pos.1] == '.' { grid[i][pos.1] = '|'; }
             else { break; }
        }
    }
}

fn split_beam(grid: &mut Vec<Vec<char>>, pos: (usize, usize)) -> bool
{
    let split: bool = grid[pos.0-1][pos.1] != '|' || grid[pos.0+1][pos.1] != '|';
    if grid[pos.0][pos.1] == '^'
    {
        place_beam(grid, (pos.0, pos.1-1));
        place_beam(grid, (pos.0, pos.1+1));
    }
    split
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    // Part 1
    let rows: Vec<&str> = lines.split('\n').filter(|x| !x.is_empty()).collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for row in rows.iter()
    {
        grid.push(row.chars().collect::<Vec<char>>());
    }

    print_grid(&grid);

    let mut num_splits = 0;
    'row_loop: for r in 0..grid.len()
    {
        for c in 0..grid[0].len()
        {
            match grid[r][c]
            {
                'S' =>
                {
                    place_beam(&mut grid, (r+1,c));
                }
                '.' | '|' =>
                {
                    continue
                }
                '^' =>
                {
                    if grid[r-1][c] == '|'
                    {
                        if split_beam(&mut grid, (r,c)) { num_splits += 1 };
                    };
                }
                _ =>
                {
                    println!("Invalid character encountered '{}'.", grid[r][c]);
                    break 'row_loop;
                }
            }
        }
    }
    print_grid(&grid);
    println!("num_splits={num_splits}");

}