use std::collections::HashSet;
use util::flatten::Flattenable;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(&input), 0)
    }
}

fn solve_a(input: &Vec<&str>) -> u32 {
    let mut banks: Vec<u32> = input.iter()
        .map(|line| {
            line.split('\t')
                .map(|s| s.parse().expect(&format!("Invalid number: {}", s)))
        })
        .flatten()
        .collect();

    let mut history: HashSet<Vec<u32>> = HashSet::new();
    let mut steps: u32 = 0;

    loop {
        if history.contains(&banks) {
            return steps;
        }

        history.insert(banks.clone());
        rebalance(&mut banks);
        steps += 1;
    }
}

fn rebalance(banks: &mut Vec<u32>) -> () {
    let (start_i, mut amount) = banks.iter()
        .enumerate()
        .fold(
            (0, 0),
            |(maxi, maxv), (i, &next)| {
                if next > maxv {
                    (i, next)
                } else {
                    (maxi, maxv)
                }
            }
        );

    banks[start_i] = 0;

    let mut i = (start_i + 1) % banks.len();
    while amount > 0 {
        banks[i] += 1;
        amount -= 1;
        i = (i + 1) % banks.len();
    }
}
