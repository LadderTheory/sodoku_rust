use rand::prelude::*;
use sudoku::sudoku::Sudoku;
use std::time::{Duration, Instant};
use clap::{Arg, App};

mod sudoku;

fn main() {
    let matches = App::new("Sudoku")
        .arg(Arg::with_name("benchmark")
            .short("b")
            .long("bench")
            .takes_value(true))
        .get_matches();

    match matches.value_of("benchmark") {
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => benchmark(n),
                Err(_) => (),
            }
        }
        _ => ()
    }

    let mut _dok: Sudoku;

    //benchmark(1000);
}

fn benchmark(arg: u32) {
    let mut all_steps = Vec::new();
    let mut times = Vec::new();
    for _i in 0..arg {
        let start = Instant::now();
        let mut s = 0;
        let p = gen_puzzle(&mut s);
        all_steps.push(s);
        times.push(start.elapsed());
        print_puzzle(&p);
    }

    let mut average: f32 = 0.0;
    let mut max = all_steps[0];
    let mut min = all_steps[0];
    let mut eighty_one = 0;
    for i in all_steps.iter() {
        average += *i as f32;
        if i > &max {
            max = *i;
        }
        if i < &min {
            min = *i;
        }
        if *i == 81 {
            eighty_one += 1;
        }
    }
    average /= all_steps.len() as f32;

    let mut average_t: Duration = times[0];
    let mut max_t = times[0];
    let mut min_t = times[0];
    for (x, i) in times.iter().enumerate() {
        if x == 0 {
            continue;
        }
        average_t += *i;
        if i > &max_t {
            max_t = *i;
        }
        if i < &min_t {
            min_t = *i;
        }
    }
    average_t /= times.len() as u32;

    println!(
        "Average steps:\t{}\nMax:\t\t{}\nMin:\t\t{}/{}",
        average, max, min, eighty_one
    );
    println!(
        "Average time elapsed:\t{:?}\nMax\t\t\t{:?}\nMin:\t\t\t{:?}",
        average_t, max_t, min_t
    );
}

fn print_puzzle(p: &Vec<u32>) {
    for (i, x) in p.iter().enumerate() {
        if i % 9 == 0 {
            println!();
        }
        let s;
        if *x == 0 {
            s = String::from("-");
        } else {
            s = format!("{}", x);
        }
        print!("{} ", s);
    }
    println!();
}

fn gen_puzzle(steps: &mut u32) -> Vec<u32> {
    let mut puzzle = vec![0; 81];
    //let mut rng = thread_rng();
    let mut pos = 0;
    let mut possible = vec![Vec::new(); 81];
    let mut backtrace = false;

    while pos < 81 {
        if !backtrace {
            possible[pos] = shuffle(get_possibilities(&puzzle, pos));
            //println!("{:?}", possible[pos]);
            if possible[pos].len() == 0 {
                backtrace = true;
                //print_puzzle(&puzzle);
                //stdin().read(&mut [0]);
            } else {
                puzzle[pos] = possible[pos][0];
            }
        } else {
            for (i, x) in possible[pos].iter().enumerate() {
                if puzzle[pos] == *x {
                    if i + 1 < possible[pos].len() {
                        puzzle[pos] = possible[pos][i + 1];
                        backtrace = false;
                    }
                    break;
                }
            }
        }

        if backtrace {
            pos -= 1;
        } else {
            pos += 1;
            *steps += 1;
        }
    }

    //print_puzzle(&puzzle);
    //println!("steps: {}", steps);

    puzzle
}

fn shuffle(arg: Vec<u32>) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut rtrn = Vec::new();
    let mut used = Vec::new();
    for _cnt in 0..arg.len() {
        let mut x = rng.gen_range(0..arg.len());
        while used.contains(&x) {
            x += 1;
            if x == arg.len() {
                x = 0;
            }
        }
        rtrn.push(arg[x]);
        used.push(x);
    }
    rtrn
}

fn get_possibilities(p: &Vec<u32>, arg_ndx: usize) -> Vec<u32> {
    //check row
    let mut pos = arg_ndx - (arg_ndx % 9);
    let mut used: Vec<u32> = Vec::new();
    for i in 0..9 {
        if pos + i != arg_ndx {
            used.push(p[pos + i]);
        } else {
            break;
        }
    }

    //check collumn
    pos = arg_ndx - ((arg_ndx / 9) * 9);
    for i in 0..9 {
        let itr = pos + (9 * i);
        if itr != arg_ndx {
            used.push(p[itr]);
        } else {
            break;
        }
    }

    //check box
    pos = arg_ndx - (arg_ndx % 3);
    pos -= ((arg_ndx / 9) % 3) * 9;
    'outer: for y in 0..3 {
        for x in 0..3 {
            let itr = pos + x + (9 * y);
            if itr != arg_ndx {
                used.push(p[itr]);
            } else {
                break 'outer;
            }
        }
    }

    let mut usable = Vec::new();
    //println!("{:?}", used);
    for i in 1..10 {
        if !used.contains(&i) {
            usable.push(i as u32);
        }
    }
    //println!("{:?}", usable);
    usable
}
