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
    println!("Hello, world!");

    println!("{:?}", day01::solve(util::stdin_lines(std::io::stdin()).into_iter()));
}
