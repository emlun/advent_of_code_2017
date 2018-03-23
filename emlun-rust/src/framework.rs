pub trait Solver {
    type A: ToString;
    type B: ToString;

    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B);
    fn solve_str(&self, input: &Vec<&str>) -> (String, String) {
        let (a, b) = self.solve(input);
        (a.to_string(), b.to_string())
    }
}
