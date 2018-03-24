use std::collections::HashSet;
use std::collections::LinkedList;
use util::flatten::Flattenable;
use util::hash::Hashable;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(&input), solve_b(&input))
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

fn solve_b(input: &Vec<&str>) -> u32 {
    let mut banks: Vec<u32> = input.iter()
        .map(|line| {
            line.split('\t')
                .map(|s| s.parse().expect(&format!("Invalid number: {}", s)))
        })
        .flatten()
        .collect();

    let mut history: HashSet<u64> = HashSet::new();
    let mut history_order: LinkedList<Vec<u32>> = LinkedList::new();

    loop {
        let hash: u64 = banks.finish_hash();
        if history.contains(&hash) {
            break;
        }

        history.insert(hash);
        history_order.push_back(banks.clone());
        rebalance(&mut banks);
    }

    return (history_order.len() - history_order.iter().position(|b| b == &banks).unwrap()) as u32;
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
