mod day01;
mod framework;

use framework::Solver;

fn day(name: &str, lines: &Vec<String>) -> i32 {
    let solver;

    match name.as_ref() {
        "day01" => solver = day01::Solver {},
        _ => {
            println!("Unknown day: {}", name);
            return 1;
        },
    }

    println!("A: {}", solver.solve_a(&lines));
    println!("B: {}", solver.solve_b(&lines));

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
