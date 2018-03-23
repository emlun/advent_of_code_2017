mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod framework;
mod util;

use framework::Solver;
use std::fs::File;
use std::io::Read;

fn day(num: u32, lines: &Vec<&str>, fluff: bool) -> i32 {
    let (a, b) = match num {
        1 => day01::Solver{}.solve_str(lines),
        2 => day02::Solver{}.solve_str(lines),
        3 => day03::Solver{}.solve_str(lines),
        4 => day04::Solver{}.solve_str(lines),
        5 => day05::Solver{}.solve_str(lines),
        6 => day06::Solver{}.solve_str(lines),
        _ => {
            println!("Unknown day: {}", num);
            return 1;
        },
    };

    if fluff {
        println!("=== Day {} ===", num);
        println!("");
    }
    println!("A: {}", a);
    println!("B: {}", b);
    if fluff {
        println!("");
    }

    0
}

fn run() -> i32 {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .map(|arg| arg.parse().expect(&format!("Argument is not a number: {}", arg)))
        .collect();

    let day_nums: Vec<u32> = if args.len() > 0 { args }
                             else { (1..7).collect() };

    for day_num in &day_nums {
        let file_name = format!("input/day{:02}.in", day_num);
        let mut f = File::open(&file_name)
            .expect(&format!("Input file for day{} not found!", day_num));

        let mut input = String::new();
        f.read_to_string(&mut input).expect(&format!("Failed to read input file {}", file_name));
        let lines: Vec<&str> = input.lines().collect();

        day(*day_num, &lines, day_nums.len() > 1);
    }

    0
}

fn main() {
    std::process::exit(run());
}
