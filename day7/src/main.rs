use std::collections::VecDeque;
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

#[derive(PartialEq, Debug, Clone)]
struct Position
{
    r: usize,
    c: usize,
}

#[derive(Debug)]
struct Node
{
    pos: Position,
    parents: Vec<Position>,
    children: Vec<Position>,
    id: usize,
    visited: bool,
}

impl Node
{
    fn new(pos: &Position, id: usize) -> Self
    {
        let parents: Vec<Position> = Vec::new();
        let children: Vec<Position> = Vec::new();
        let new_pos = pos.clone();
        Node{ pos: new_pos, parents, children, id, visited: false }
    }

    fn add_child(&mut self, pos: &Position)
    {
        if !self.children.contains(&pos)
        {
            let new_pos = pos.clone();
            self.children.push(new_pos);
        }
    }

    fn add_parent(&mut self, pos: &Position)
    {
        if !self.parents.contains(&pos)
        {
            let new_pos = pos.clone();
            self.parents.push(new_pos);
        }
    }
}

#[derive(Debug)]
struct Tree
{
    num_nodes: usize,
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl Tree
{
    fn new() -> Self
    {
        let num_nodes = 0;
        let nodes: Vec<Node> = Vec::new();
        let edges: Vec<(usize, usize)> = Vec::new();
        Tree{ num_nodes, nodes, edges }
    }

    fn add_node(&mut self, node: Node)
    {
        if self.nodes.iter().find(|x| x.pos == node.pos).is_none()
        {
            self.nodes.push(node);
            self.num_nodes += 1;
        }
    }

    fn add_edge(&mut self, parent: Position, child: Position)
    {
        let mut found_parent = false;
        let mut found_child = false;
        let mut edge: (usize, usize) = (0, 0);
        for node in self.nodes.iter_mut()
        {
            if node.pos == parent
            {
                node.add_child(&child);
                edge.0 = node.id;
                found_parent = true;
            }
            if node.pos == child
            {
                node.add_parent(&parent);
                edge.1 = node.id;
                found_child = true;
            }
        }

        if !(found_parent && found_child)
        {
            println!("BAD EDGE detected");
        }
        else
        {
            self.edges.push(edge);
        }
    }

    fn total_paths(&mut self) -> u64
    {
        let mut num: u64 = 0;

        // Initialize all nodes to not visited
        for node in self.nodes.iter_mut()
        {
            node.visited = false;
        }

        // Create adjacency list
        let mut adj_list: Vec<Vec<usize>> = Vec::new();
        let mut input_list = vec![0; self.num_nodes];
        for _ in 0..self.num_nodes
        {
            let list: Vec<usize> = Vec::new();
            adj_list.push(list);
        }

        for (u, v) in self.edges.iter()
        {
            adj_list[*u].push(*v);
            input_list[*v] += 1;
        }

        // Perform topological sort using Kahn's algorithm
        let mut q: VecDeque<usize> = VecDeque::new();
        for i in 0..self.num_nodes
        {
            if input_list[i] == 0 { q.push_front(i); }
        }

        let mut topo_order: Vec<usize> = Vec::new();

        loop
        {
            let id_opt = q.pop_back();

            match id_opt
            {
                None => { break; }

                Some(x) =>
                {
                    topo_order.push(x);

                    for neighbor in adj_list[x].iter()
                    {
                        input_list[*neighbor] -= 1;
                        if input_list[*neighbor] == 0
                        {
                            q.push_front(*neighbor);
                        }
                    }
                }
            }
        }

        // Traverse topological order
        let mut ways = vec![0; self.num_nodes];
        ways[0] = 1;

        for id in topo_order.iter()
        {
            for neighbor in adj_list[*id].iter()
            {
                ways[*neighbor] += ways[*id];
            }
        }

        // Accumulate number of ways for all end nodes
        for node in self.nodes.iter()
        {
            if node.children.len() == 0
            {
                num += ways[node.id];
            }
        }

        // println!("top_order = {topo_order:?}");
        // println!("top_order.len = {}", topo_order.len());

        num
    }
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

    // Part 1: Build tree and count splits
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

    // Part 2: Build tree and count possible paths
    let mut tree= Tree::new();
    'row_loop: for r in 0..grid.len()
    {
        for c in 0..grid[0].len()
        {
            match grid[r][c]
            {
                'S' =>
                {
                    let mut next_pos = Position{ r, c };
                    // Create first link in tree
                    loop
                    {
                        let x = grid.get(next_pos.r).and_then(|y| y.get(next_pos.c));

                        match x
                        {
                            None => break,

                            Some('^') =>
                            {
                                let cur_pos = Position{r,c};
                                let root = Node::new(&cur_pos, tree.num_nodes);
                                tree.add_node(root);
                                let node = Node::new(&next_pos, tree.num_nodes);
                                tree.add_node(node);
                                tree.add_edge(cur_pos, next_pos);

                                break;
                            }

                            Some(_) => next_pos.r += 1,
                        }
                    }
                }
                '.' | '|' =>
                {
                    continue
                }
                '^' =>
                {
                    if grid[r-1][c] == '|'
                    {
                        // Create link from left split in tree
                        let cols = vec![c-1, c+1];
                        for col in cols
                        {
                            let mut next_pos = Position{ r, c: col };
                            loop
                            {
                                let x = grid.get(next_pos.r).and_then(|y| y.get(next_pos.c));

                                match x
                                {
                                    None | Some('^') =>
                                    {
                                        let cur_pos = Position{r,c};
                                        let node = Node::new(&next_pos, tree.num_nodes);
                                        tree.add_node(node);
                                        tree.add_edge(cur_pos, next_pos);
                                        break;
                                    }

                                    Some(_) => next_pos.r += 1,
                                }
                            }
                        }
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
    println!("tree.num_nodes = {}", tree.num_nodes);
    print_grid(&grid);
    println!("total_paths={}", tree.total_paths());

}