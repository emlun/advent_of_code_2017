use std::io::BufRead;

pub fn stdin_lines(stdin: ::std::io::Stdin) -> Vec<String> {
    stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line from stdin"))
        .collect()
}

pub trait Solver<A, B>
    where A: ToString,
          B: ToString,
{
    fn solve_a(&self, input: &Vec<String>) -> A;
    fn solve_b(&self, input: &Vec<String>) -> B;
}
