fn read_digits(input: &Vec<String>) -> Vec<u32> {
    input
        .iter()
        .flat_map::<Vec<char>, _>(|line| line.chars().collect())
        .map(|c: char| c.to_digit(10).expect("Invalid digit"))
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
    digits
        .iter()
        .enumerate()
        .fold(
            0,
            |sum, (i, &a)| {
                let b = digits[(i + 1) % digits.len()];
                if a == b { sum + a }
                else { sum }
            }
        )
}

fn solve_b(digits: &Vec<u32>) -> u32 {
    digits
        .iter()
        .enumerate()
        .fold(
            0,
            |sum, (i, &a)| {
                let b = digits[(i + digits.len() / 2) % digits.len()];
                if a == b { sum + a }
                else { sum }
            }
        )
}
