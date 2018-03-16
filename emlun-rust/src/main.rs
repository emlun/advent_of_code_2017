mod day01;
mod framework;

use framework::Solver;

fn day(name: &str, lines: &Vec<String>) -> i32 {
    let (a, b) = match name.as_ref() {
        "day01" => day01::Solver{}.solve_str(&lines),
        _ => {
            println!("Unknown day: {}", name);
            return 1;
        },
    };

    println!("A: {}", a);
    println!("B: {}", b);

    0
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().collect();
    let lines = framework::stdin_lines(std::io::stdin());

    if args.len() > 1 {
        day(args[1].as_ref(), &lines)
    } else {
        for d in ["01"].iter() {
            day(d.as_ref(), &lines);
        }
        0
    }
}

fn main() {
    std::process::exit(run());
}