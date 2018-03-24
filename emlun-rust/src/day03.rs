use util;
use util::flatten::Flattenable;
use std::cmp::max;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(&input), solve_b(input))
    }
}

fn btm_right(level: u32) -> u32 {
    1 + 4 * (level * (level + 1))
}

/** Solve `i <= btm_right(l)` for `l` using the quadratic roots formula */
fn find_level(i: u32) -> u32  {
    (0.5 * ((i as f64).sqrt() - 1.0)).ceil() as u32
}

fn solve_generic<F>(input: &Vec<&str>, solve: F) -> u32
    where F: (Fn(u32) -> u32)
{
  let cell_num: u32 = input[0].trim().parse().expect(&format!("Invalid decimal number: {}", input[0]));
  solve(cell_num)
}

fn solve_a(input: &Vec<&str>) -> u32 {
  solve_generic(input, dist)
}

fn dist(i: u32) -> u32 {
    let level: i32 = find_level(i) as i32;
    let index_in_circle: i32 = (i as i32) - (btm_right(max(0, level as u32 - 1)) as i32) - (level as i32);
    let modulus: i32 = max(1, 2 * level);

    (level + (index_in_circle % modulus).abs()) as u32
}

fn shell(prev: &Vec<u32>) -> Vec<u32> {
    let level: usize = prev.len() / 8 + 1;

    if level < 2 {
      vec![1, 2, 4, 5, 10, 11, 23, 25]
    } else {
        let length: usize = prev.len() + 8;
        let m = 2 * level - 1;

        fn value(prev: &Vec<u32>, m: usize, i: usize) -> u32 {
            let prevlast = prev.last().unwrap();
            let recur = |i: usize| value(prev, m, i);
            match i {
                0                       =>                prevlast    + prev[i],
                1                       => prevlast     + prev[i - 1] + prev[i]     + recur(i - 1),
                i if i < m-1            => prev[i - 2]  + prev[i - 1] + prev[i]     + recur(i - 1),
                i if i == m-1           => prev[i - 2]  + prev[i - 1]               + recur(i - 1),
                i if i == m             => prev[i - 2]                              + recur(i - 1),
                i if i == m+1           => recur(i - 2) + prev[i - 3] + prev[i - 2] + recur(i - 1),

                i if i < m+m            => prev[i - 4]  + prev[i - 3] + prev[i - 2] + recur(i - 1),
                i if i == m+m           => prev[i - 4]  + prev[i - 3]               + recur(i - 1),
                i if i == m+m+1         => prev[i - 4]                              + recur(i - 1),
                i if i == m+m+1+1       => recur(i - 2) + prev[i - 5] + prev[i - 4] + recur(i - 1),

                i if i < m+m+1+m        => prev[i - 6]  + prev[i - 5] + prev[i - 4] + recur(i - 1),
                i if i == m+m+1+m       => prev[i - 6]  + prev[i - 5]               + recur(i - 1),
                i if i == m+m+1+m+1     => prev[i - 6]                              + recur(i - 1),
                i if i == m+m+1+m+1+1   => recur(i - 2) + prev[i - 7] + prev[i - 6] + recur(i - 1),

                i if i < m+m+1+m+1+m    => prev[i - 8]  + prev[i - 7] + prev[i - 6] + recur(i - 1),
                i if i == m+m+1+m+1+m   => prev[i - 8]  + prev[i - 7] + recur(0)    + recur(i - 1),
                i if i == m+m+1+m+1+m+1 => prev[i - 8]  + recur(0)                  + recur(i - 1),
                _ => panic!("Index {} should not be possible", i)
            }
        };

        (0..length)
            .map(|v| value(&prev, m, v))
            .collect()
    }
}

fn solve_b(input: &Vec<&str>) -> u32 {
    solve_generic(
        input,
        |cell_num: u32|
            util::iterate::iterate(vec![1], shell)
                .flatten()
                .find(|&a| a > cell_num)
                .unwrap()
    )
}
