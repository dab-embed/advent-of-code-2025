use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct MathProblem {
   numbers: Vec<u64>,
   operation: char 
}

impl MathProblem
{
    fn new() -> Self
    {
        let numbers: Vec<u64> = Vec::new();
        MathProblem
        {
            numbers: numbers,
            operation: '+'
        }   
    }

    fn push(&mut self, number: u64)
    {
        self.numbers.push(number);
    }

    fn set_op(&mut self, op: char) -> bool
    {
        let valid = ['*', '+'].contains(&op);
        if valid { self.operation = op }
        valid
    }

    fn solve(&self) -> u64
    {
        let mut value: u64 = 0;
        match self.operation
        {
            '+' =>
            {
                value = self.numbers.iter().sum();
            }
            '*' =>
            { 
                value = self.numbers.iter().product();
            }
            _ =>
            {
                println!("No valid operation.")
            }
        }
        value
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    // Part 1
    let rows: Vec<&str> = lines.split('\n').filter(|x| !x.is_empty()).collect();
    let mut problems: Vec<MathProblem> = Vec::new();
    let mut error: bool = false;
    for row in rows.iter()
    {
        let items: Vec<&str> = row.split(' ').filter(|x| !x.is_empty()).collect();

        for (index, item) in items.iter().enumerate()
        {
            if problems.len() <= index
            {
                let problem: MathProblem = MathProblem::new();
                problems.push(problem);
            }

            let number = item.parse::<u64>();

            match number
            {
                Ok(n) => 
                    problems[index].push(n),
                Err(_) =>
                if !problems[index].set_op(item.chars().nth(0).unwrap()) 
                {   
                    println!("Not a valid operation.");
                    error = true;
                    break;
                },
            }
        }
    }
    //println!{"problems = {problems:#?}"};

    if !error
    {
        let mut total = 0;
        for problem in problems.iter()
        {
            total += problem.solve();
        }
        println!("Total(part1)={total}");
    }

    // Part 2
    let rows: Vec<&str> = lines.split('\n').filter(|x| !x.is_empty()).collect();
    let total_columns = rows.iter().nth(0).unwrap().len() - 1;
    let mut columns: Vec<String> = Vec::new();
    let mut error = false;
    for _ in 0..=total_columns { columns.push(String::new()); }

    for row in rows[0..rows.len()-1].iter()
    {
        for (i, c) in row[0..=total_columns].chars().enumerate()
        {
            columns[i].push(c);
        }
    }

    let mut problems: Vec<MathProblem> = Vec::new();    
    for op in rows[rows.len()-1].split(' ').filter(|x|!x.is_empty())
    {
        let mut problem = MathProblem::new();
        if !problem.set_op(op.chars().nth(0).unwrap())
        {
            println!("Not a valid operation.");
            error = true;
            break;
        }

        problems.push(problem);
    }

    if !error
    {
        let mut p = 0;
        for column in columns.iter()
        {
            let value: String = column.split(' ').filter(|x|!x.is_empty()).collect();

            if !value.is_empty()
            {
                let value= value.parse::<u64>();

                match value
                {
                    Ok(v) =>
                    {
                        problems[p].push(v);
                    },
                    Err(_) =>
                    {
                        error = true;
                        println!("Not a valid number.");
                        break;
                    },
                }
            }
            else
            {
                p += 1;
            }
        }
    }

    //println!{"problems = {problems:#?}"};
    if !error
    {
        let mut total = 0;
        for problem in problems.iter()
        {
            total += problem.solve();
        }
        println!("Total(part2)={total}");
    }
}