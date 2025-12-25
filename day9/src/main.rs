use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Position
{
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Rectangle
{
    a: Position,
    b: Position,
    area: u64,
}

impl Rectangle
{
    fn new(a: &Position, b: &Position) -> Self
    {
        let len = a.x;
        let len = (len.abs_diff(b.x) + 1) as u64;
        let height = a.y;
        let height = (height.abs_diff(b.y) + 1) as u64;
        let area: u64 = len * height;
        Rectangle { a: a.clone(), b: b.clone(), area }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    // Part 1
    let rows: Vec<&str> = lines.split('\n').filter(|x| !x.is_empty()).collect();
    let mut positions: Vec<Position> = Vec::new();

    // Create list of positions
    for row in rows.iter()
    {
        let vals: Vec<usize> = row.split(',').filter(|x| !x.is_empty()).map(|val| val.parse::<usize>().unwrap()).collect();
        let position = Position{ x: vals[0], y: vals[1]};
        positions.push(position);
    }

    // Create list of rectangles
    let mut rectangles: Vec<Rectangle> = Vec::new();
    loop
    {
        for i in 1..positions.len()
        {
            let rectangle = Rectangle::new(&positions[0], &positions[i]);
            rectangles.push(rectangle);
        }
        positions.remove(0);
        if positions.is_empty() { break; }
    }

    let mut max_area = 0;
    for rectangle in rectangles.iter()
    {
        if max_area < rectangle.area
        {
            max_area = rectangle.area;
        }
    }

    println!("result(part1)={}", max_area);

}


