use std::io::BufRead;

pub fn stdin_lines(stdin: ::std::io::Stdin) -> Vec<String> {
    stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line from stdin"))
        .collect()
}

pub trait Solver {
    type A: ToString;
    type B: ToString;

    fn solve(&self, input: &Vec<String>) -> (Self::A, Self::B);
}
