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
    fn solve_str(&self, input: &Vec<String>) -> (String, String) {
        let (a, b) = self.solve(input);
        (a.to_string(), b.to_string())
    }
}
