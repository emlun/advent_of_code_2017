mod day01;

mod util {
    use std::io::BufRead;

    pub fn stdin_lines(stdin: ::std::io::Stdin) -> Vec<String> {
        // ::std::io::stdin()
        stdin
            .lock()
            .lines()
            .map(|line| line.expect("Failed to read line from stdin"))
            .collect()
    }
}

fn main() {
    let lines = util::stdin_lines(std::io::stdin());
    println!("A: {}", day01::solve_a(&lines));
    println!("B: {}", day01::solve_b(&lines));
}
