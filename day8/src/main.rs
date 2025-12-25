use std::env;
use std::fs::read_to_string;

#[derive(PartialEq, Debug, Clone)]
struct Position
{
    x: usize,
    y: usize,
    z: usize,
    id: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct Edge
{
    a: Position,
    b: Position,
    len: f64,
}

impl Edge
{
    fn new(a: &Position, b: &Position) -> Self
    {
        let len: f64 = f64::sqrt((
            i64::pow( (a.x as i64) - (b.x as i64), 2 ) +
            i64::pow( (a.y as i64) - (b.y as i64), 2 ) +
            i64::pow( (a.z as i64) - (b.z as i64), 2 ) ) as f64 );

        Edge{ a: a.clone(), b: b.clone(), len }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let num_connections= &args[2].parse::<usize>().expect("User needs to specify number of desired connections as 2nd argument.");
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    // Part 1
    let rows: Vec<&str> = lines.split('\n').filter(|x| !x.is_empty()).collect();
    let mut positions: Vec<Position> = Vec::new();

    // Create list of positions
    for (idx, row) in rows.iter().enumerate()
    {
        let vals: Vec<usize> = row.split(',').filter(|x| !x.is_empty()).map(|val| val.parse::<usize>().unwrap()).collect();
        let position = Position{ x: vals[0], y: vals[1], z: vals[2], id: idx};
        positions.push(position);
    }
    let mut circuit_counts: Vec<u64> = vec![1; positions.len()];
    let mut edges: Vec<Edge> = Vec::new();
    let mut sorted_edges: Vec<Edge> = Vec::new();

    // Create list of independent edges
    loop
    {
        for i in 1..positions.len()
        {
            let edge = Edge::new(&positions[0], &positions[i]);
            edges.push(edge);
        }
        positions.remove(0);
        if positions.is_empty() { break; }
    }

    // Sort the first N edges based on length (N specified by user)
    // Make specified number of connections
    for n in 0..*num_connections
    {
        // Sort the next shortest length
        let mut min_len = edges[0].len;
        let mut min_idx = 0;
        for (idx, edge) in edges.iter().enumerate()
        {
            if edge.len < min_len
            {
                min_len = edge.len;
                min_idx = idx;
            }
        }
        sorted_edges.push(edges.remove(min_idx));

        // Make the next connection
        let keep_id = sorted_edges[n].a.id;
        let remove_id = sorted_edges[n].b.id;
        if keep_id != remove_id
        {
            // println!("connecting circuit[{}]={} with circuit[{}]={}", keep_id, circuit_counts[keep_id], remove_id, circuit_counts[remove_id]);
            circuit_counts[keep_id] += circuit_counts[remove_id];
            circuit_counts[remove_id] = 0;

            // Clean up IDs of deleted circuits
            for edge in sorted_edges.iter_mut()
            {
                if edge.a.id == remove_id { edge.a.id = keep_id; }
                if edge.b.id == remove_id { edge.b.id = keep_id; }
            }
            for edge in edges.iter_mut()
            {
                if edge.a.id == remove_id { edge.a.id = keep_id; }
                if edge.b.id == remove_id { edge.b.id = keep_id; }
            }
        }
    }

    let mut result: u64 = 1;
    let mut tmp_circuit_counts = circuit_counts.clone();
    for _ in 0..3
    {
        let max_index = tmp_circuit_counts.iter().enumerate().max_by_key(|&(_, &val)| val).map(|(index, _)| index).unwrap();
        result *= tmp_circuit_counts.remove(max_index);
    }
    println!("result(part1) = {result:?}");

    // Part 2, keep connecting until all on once circuit
    let mut n = *num_connections;
    loop
    {
        // Sort the next shortest length
        let mut min_len = edges[0].len;
        let mut min_idx = 0;
        for (idx, edge) in edges.iter().enumerate()
        {
            if edge.len < min_len
            {
                min_len = edge.len;
                min_idx = idx;
            }
        }
        sorted_edges.push(edges.remove(min_idx));

        // Make the next connection
        let keep_id = sorted_edges[n].a.id;
        let remove_id = sorted_edges[n].b.id;
        if keep_id != remove_id
        {
            // println!("connecting circuit[{}]={} with circuit[{}]={}", keep_id, circuit_counts[keep_id], remove_id, circuit_counts[remove_id]);
            circuit_counts[keep_id] += circuit_counts[remove_id];
            circuit_counts[remove_id] = 0;

            for edge in sorted_edges.iter_mut()
            {
                if edge.a.id == remove_id { edge.a.id = keep_id; }
                if edge.b.id == remove_id { edge.b.id = keep_id; }
            }
            for edge in edges.iter_mut()
            {
                if edge.a.id == remove_id { edge.a.id = keep_id; }
                if edge.b.id == remove_id { edge.b.id = keep_id; }
            }
        }
        let mut num_circuits = 0;
        for count in circuit_counts.iter()
        {
            if *count != 0 { num_circuits += 1; }
        }
        if num_circuits == 1
        {
            let result = sorted_edges[n].a.x * sorted_edges[n].b.x;
            println!("last junction edge = {:?}", sorted_edges[n]);
            println!("result(part2)={result}");
            break;
        }
        n += 1;
    }
}
