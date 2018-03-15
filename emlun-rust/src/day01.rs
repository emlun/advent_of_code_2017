fn read_digits(input: &Vec<String>) -> Vec<u32> {
    input
        .iter()
        .flat_map::<Vec<char>, _>(|line| line.chars().collect())
        .map(|c: char| c.to_digit(10).expect("Invalid digit"))
        .collect()
}

pub struct Solver {}

impl ::framework::Solver<u32, u32> for Solver {
    fn solve_a(&self, input: &Vec<String>) -> u32 {
        let digits: Vec<u32> = read_digits(input);
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

    fn solve_b(&self, input: &Vec<String>) -> u32 {
        let digits: Vec<u32> = read_digits(input);
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
}
