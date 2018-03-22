use std::collections::HashSet;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = usize;
    type B = usize;
    fn solve(&self, input: &Vec<String>) -> (Self::A, Self::B) {
        (solve_a(&input), solve_b(&input))
    }
}

fn solve_a(input: &Vec<String>) -> usize {
    input.iter().filter(|s| is_valid_passphrase(s)).count()
}

fn is_valid_passphrase(passphrase: &String) -> bool {
    let words: Vec<&str> = passphrase.split(' ').collect();
    let deduplicated: HashSet<&&str> = words.iter().collect();
    deduplicated.len() == words.len()
}

fn solve_b(input: &Vec<String>) -> usize {
    input.iter().filter(|s| is_extra_valid_passphrase(s)).count()
}

fn is_extra_valid_passphrase(passphrase: &String) -> bool {
    let words: Vec<Vec<char>> = passphrase
        .split(' ')
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            chars
        })
        .collect();
    let deduplicated: HashSet<&Vec<char>> = words.iter().collect();
    deduplicated.len() == words.len()
}
