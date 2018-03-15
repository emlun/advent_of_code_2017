fn read_digits(input: &Vec<String>) -> Vec<u32> {
    input
        .iter()
        .flat_map::<Vec<char>, _>(|line| line.chars().collect())
        .map(|c: char| c.to_digit(10).expect(&format!("Invalid digit: {}", c)))
        .collect()
}

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;

    fn solve(&self, input: &Vec<String>) -> (u32, u32) {
        let digits: Vec<u32> = read_digits(input);
        (solve_a(&digits), solve_b(&digits))
    }
}

fn solve_a(digits: &Vec<u32>) -> u32 {
    solve_generic(digits, 1)
}

fn solve_b(digits: &Vec<u32>) -> u32 {
    solve_generic(digits, digits.len() / 2)
}

fn solve_generic(digits: &Vec<u32>, lookahead: usize) -> u32 {
    digits
        .iter()
        .enumerate()
        .fold(
            0,
            |sum, (i, &a)| {
                let b = digits[(i + lookahead) % digits.len()];
                if a == b { sum + a }
                else { sum }
            }
        )
}
