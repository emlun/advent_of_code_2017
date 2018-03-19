use std::cmp::max;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;
    fn solve(&self, input: &Vec<String>) -> (Self::A, Self::B) {
        (solve_a(&input), solve_a(&input))
    }
}

fn btm_right(level: u32) -> u32 {
    1 + 4 * (level * (level + 1))
}

/** Solve `i <= btm_right(l)` for `l` using the quadratic roots formula */
fn find_level(i: u32) -> u32  {
    (0.5 * ((i as f64).sqrt() - 1.0)).ceil() as u32
}

fn solve_a(input: &Vec<String>) -> u32 {
  let cell_num: u32 = input[0].trim().parse().expect(&format!("Invalid decimal number: {}", input[0]));
  dist(cell_num)
}

fn dist(i: u32) -> u32 {
    let level: i32 = find_level(i) as i32;
    let index_in_circle: i32 = (i as i32) - (btm_right(max(0, level as u32 - 1)) as i32) - (level as i32);
    let modulus: i32 = max(1, 2 * level);

    (level + (index_in_circle % modulus).abs()) as u32
}
