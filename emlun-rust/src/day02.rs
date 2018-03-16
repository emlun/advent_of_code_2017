pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = i32;
    type B = i32;
    fn solve(&self, input: &Vec<String>) -> (Self::A, Self::B) {
        let spreadsheet = parse_spreadsheet(input);
        (solve_a(&spreadsheet), solve_b(&spreadsheet))
    }
}

fn parse_spreadsheet(lines: &Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line
                .split(|c| c == ' ' || c == '\t')
                .filter(|s| !s.is_empty())
                .map(|word|
                     word.parse().expect(&format!("Invalid number: {}", word))
                )
                .collect()
        })
        .collect()
}

fn solve_a(spreadsheet: &Vec<Vec<i32>>) -> i32 {
    spreadsheet
        .iter()
        .map(|nums| {
            let first = nums.first().unwrap();

            let minmax = nums
                .iter()
                .fold(
                    (first, first),
                    |(min, max), next|
                        if next < min      { (next, max) }
                        else if next > max { (min, next) }
                        else               { (min, max)  }
                );
            minmax.1 - minmax.0
        })
        .sum()
}

fn solve_b(spreadsheet: &Vec<Vec<i32>>) -> i32 {
    spreadsheet
        .iter()
        .map(|nums| {
            let (a, b) = nums
                .iter()
                .enumerate()
                .map(|(i, &a)| {
                    nums[(i + 1)..]
                        .iter()
                        .find(|&b| a % b == 0 || b % a == 0)
                        .map(|&b| (a, b))
                })
                .find(|result| result.is_some())
                .expect(&format!("Expected to find two evenly divisible numbers in line: {:?}", nums))
                .unwrap()
            ;
            ::std::cmp::max(a, b) / ::std::cmp::min(a, b)
        })
        .sum()
}
