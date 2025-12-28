use std::{env, i64};
use std::fs::read_to_string;
use std::cmp::{min, max};

#[derive(PartialEq, Debug, Clone)]
struct Position
{
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Edge
{
    a: Position,
    b: Position,
}

impl Edge
{
    fn intersects_with(&self, other: &Edge) -> bool
    {
        let mut intersects = true;
        // Determine if edges intersect through differing signed cross products
        let cross_1a = (self.a.x-other.a.x)*(self.b.y-other.a.y) - (self.b.x-other.a.x)*(self.a.y-other.a.y);
        let cross_1b = (self.a.x-other.b.x)*(self.b.y-other.b.y) - (self.b.x-other.b.x)*(self.a.y-other.b.y);
        let cross_2a = (other.a.x-self.a.x)*(other.b.y-self.a.y) - (other.b.x-self.a.x)*(other.a.y-self.a.y);
        let cross_2b = (other.a.x-self.b.x)*(other.b.y-self.b.y) - (other.b.x-self.b.x)*(other.a.y-self.b.y);

        intersects &= (cross_1a.is_negative() && cross_1b.is_positive()) || (cross_1a.is_positive() && cross_1b.is_negative());
        intersects &= (cross_2a.is_negative() && cross_2b.is_positive()) || (cross_2a.is_positive() && cross_2b.is_negative());

        // Check special co-linear case
        // We define "colinear intersection" to be only if self is colinear with other
        // AND self is not fully contained within other
        if cross_1a == 0 && cross_1b == 0 && cross_2a == 0 && cross_2b == 0
        {
            intersects = true;
            println!("{self:?} is co-linear with {other:?}");

            intersects &= !((min(other.a.x,other.b.x)..=max(other.a.x,other.b.x)).contains(&self.a.x) &
                            (min(other.a.x,other.b.x)..=max(other.a.x,other.b.x)).contains(&self.b.x) &
                            (min(other.a.y,other.b.y)..=max(other.a.y,other.b.y)).contains(&self.a.y) &
                            (min(other.a.y,other.b.y)..=max(other.a.y,other.b.y)).contains(&self.b.y)
                           );
            if intersects
            {
                println!("{self:?} is co-linear intersecting with {other:?}");
            }
        }

        if intersects
        {
            println!("{self:?} intersects with {other:?}");
            println!("cross_1a=({}-{})*({}-{}) - ({}-{})*({}-{}) = {}", self.a.x, other.a.x, self.b.y, other.a.y, self.b.x, other.a.x, self.a.y, other.a.y, cross_1a);
            println!("cross_1b=({}-{})*({}-{}) - ({}-{})*({}-{}) = {}", self.a.x, other.b.x, self.b.y, other.b.y, self.b.x, other.b.x, self.a.y, other.b.y, cross_1b);
            println!("cross_2a=({}-{})*({}-{}) - ({}-{})*({}-{}) = {}", other.a.x, self.a.x, other.b.y, self.a.y, other.b.x, self.a.x, other.a.y, self.a.y, cross_2a);
            println!("cross_2b=({}-{})*({}-{}) - ({}-{})*({}-{}) = {}", other.a.x, self.b.x, other.b.y, self.b.y, other.b.x, self.b.x, other.a.y, self.b.y, cross_2b);
        }
        else
        {
            println!("{self:?} does not intersect with {other:?}");
        }
        intersects
    }
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

    fn get_edges(&self) -> Vec<Edge>
    {
        let mut edges: Vec<Edge> = Vec::new();

        let c = Position{ x: self.a.x, y: self.b.y }; 
        let d = Position{ x: self.b.x, y: self.a.y }; 

        edges.push( Edge{ a: self.a.clone(), b: c.clone() } );
        edges.push( Edge{ a: self.b.clone(), b: c.clone() } );
        edges.push( Edge{ a: self.a.clone(), b: d.clone() } );
        edges.push( Edge{ a: self.b.clone(), b: d.clone() } );

        edges
    }
}

struct Polygon
{
    edges: Vec<Edge>,
}

impl Polygon
{
    fn new() -> Self
    {
        let edges: Vec<Edge> = Vec::new();
        Polygon{ edges }
    }

    fn is_closed(&self) -> bool
    {
        !(self.edges.len() < 3 && (self.edges.first().unwrap().a != self.edges.last().unwrap().b))
    }

    fn is_inside(&self, pos: &Position) -> bool
    {
        let mut inside = false;

        // Check if point exists inside of any of the polygon edges
        // NOTE: only works if the polygon is formed from right angles (which is this puzzle)
        for edge in self.edges.iter()
        {
            inside |=  (min(edge.a.x,edge.b.x)..=max(edge.a.x,edge.b.x)).contains(&pos.x) &&
                       (min(edge.a.y,edge.b.y)..=max(edge.a.y,edge.b.y)).contains(&pos.y);

            // Check if point (via raycastng) is inside or outside
            if !inside
            {
                let edge = Edge{ a: pos.clone(), b: Position{ x: pos.x, y: i64::MAX }};
                
            }
        }


        inside
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    let rows: Vec<&str> = lines.split('\n').filter(|x| !x.is_empty()).collect();
    let mut positions: Vec<Position> = Vec::new();

    // Create list of positions of all red tiles
    for row in rows.iter()
    {
        let vals: Vec<i64> = row.split(',').filter(|x| !x.is_empty()).map(|val| val.parse::<i64>().unwrap()).collect();
        let position = Position{ x: vals[0], y: vals[1]};
        positions.push(position);
    }

    // Create fence around red tile perimeter where green tiles will fill in
    let mut fence: Vec<Edge> = Vec::new();
    for i in 1..positions.len()
    {
        let edge = Edge{ a: positions[i-1].clone(), b: positions[i].clone() };
        fence.push(edge);
    }
    println!("fence={fence:#?}");

    // Part 1: Create list of rectangles
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

    // Get largest possible rectangle
    let mut max_area = 0;
    for rectangle in rectangles.iter()
    {
        if max_area < rectangle.area
        {
            max_area = rectangle.area;
        }
    }

    println!("result(part1)={}", max_area);

    // Part 2: Get largest possible rectangle that fits within fence
    let mut max_area = 0;
    for rectangle in rectangles.iter()
    {
        let mut intersects = false;
        let edges = rectangle.get_edges();

        println!("rectangle={rectangle:?}");
        // println!("edges={edges:?}");
        for r_edge in edges.iter()
        {
            for f_edge in fence.iter()
            {
                intersects |= r_edge.intersects_with(&f_edge);
            }
        }

        if !intersects && max_area < rectangle.area
        {
            max_area = rectangle.area;
            println!("NEW MAX AREA = {}", max_area);
        }
    }

    println!("result(part2)={}", max_area);
}


