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

    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B);
    fn solve_str(&self, input: &Vec<&str>) -> (String, String) {
        let (a, b) = self.solve(input);
        (a.to_string(), b.to_string())
    }
}
