use std::env;
use std::fs::read_to_string;

const NUM_DAIL_POS: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_to_string(file_path).expect("Couldn't read file.");

    let mut pos: i32 = NUM_DAIL_POS/2;
    let mut nzl: i32 = 0;
    let mut nzp: i32 = 0;

    for line in lines.split('\n').filter(|x|!x.is_empty()) {
        let mut amount = (&line[1..]).parse().unwrap();
        if line.as_bytes()[0] == b'L' 
        {
            amount *= -1;
        }

        let (newpos, zl, zp)= turn_dail(pos, amount);
        nzl += zl;
        nzp += zp;
        //println!("{line:<5}:\tpos(b4)={pos:<3},amount={amount:<4},pos={newpos:<3},nzl={nzl:<5},nzp={nzp:<5}");
        pos = newpos;
    }
    println!("zeros landed={nzl},zeros passed={nzp}");
}

fn turn_dail(pos: i32, amount: i32) -> (i32, i32, i32)
{
    let mut newpos = pos;
    let mut zp = 0;
    let full_spins = amount / NUM_DAIL_POS;
    let inner_spin = amount - (full_spins*NUM_DAIL_POS);
    
    newpos += inner_spin;
    if pos > 0 && (newpos <= 0 || newpos >= NUM_DAIL_POS )
    {
        zp += 1;
    }
    zp += full_spins.abs();

    if newpos < 0
    {
        newpos += NUM_DAIL_POS;
    }
    else if newpos >= NUM_DAIL_POS
    {
        newpos -= NUM_DAIL_POS;
    }

    ( newpos, (newpos == 0) as i32, zp )
}

