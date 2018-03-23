pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(&input), solve_b(&input))
    }
}

fn solve_generic<F>(input: &Vec<&str>, mutate: F) -> u32
    where F: Fn(usize, &mut Vec<i32>) -> ()
{
    let mut jumps: Vec<i32> = input.iter()
        .map(|i| i.parse().expect(&format!("Invalid number: {}", i)))
        .collect();

    let mut steps: u32 = 0;
    let mut i: i32 = 0;

    while i >= 0 && i < (jumps.len() as i32) {
        let diff = jumps[i as usize];
        mutate(i as usize, &mut jumps);
        i += diff;
        steps += 1;
    }

    steps
}

fn solve_a(input: &Vec<&str>) -> u32 {
    solve_generic(input, |i, jumps| jumps[i] += 1)
}

fn solve_b(input: &Vec<&str>) -> u32 {
    solve_generic(
        input,
        |i, jumps| {
            jumps[i as usize] += if jumps[i] >= 3 { -1 }
                                 else { 1 };
        }
    )
}
