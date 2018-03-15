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
        let mut digits: Vec<u32> = read_digits(input);
        let first: u32 = *digits.first().unwrap();
        digits.push(first);

        digits
            .windows(2)
            .fold(
                0,
                |sum, window| {
                    let a = window[0];
                    let b = window[1];
                    if a == b { sum + a }
                    else { sum }
                }
            )
    }

    fn solve_b(&self, input: &Vec<String>) -> u32 {
        let digits: Vec<u32> = read_digits(input);
        let mut digits_twice = digits.clone();
        digits_twice.append(&mut digits.clone());

        digits
            .iter()
            .enumerate()
            .fold(
                0,
                |sum, (i, a)| {
                    let b = digits[(i + digits.len() / 2) % digits.len()];
                    if *a == b { sum + a }
                    else { sum }
                }
            )
    }
}
