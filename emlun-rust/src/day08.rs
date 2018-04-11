use std::collections::HashMap;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = i32;
    type B = i32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(input), solve_b(input))
    }
}

fn execute_instruction(inst: &str, state: &mut HashMap<String, i32>) {

    let mut parts = inst.split(" ");

    let reg = String::from(parts.next().unwrap());
    let op = parts.next().unwrap();
    let diff: i32 = parts.next().unwrap().parse().unwrap();
    parts.next();
    let test = parts.next().unwrap();
    let test_op = parts.next().unwrap();
    let test_cmp: i32 = parts.next().unwrap().parse().unwrap();

    let current_test: i32 = *state.entry(String::from(test)).or_insert(0);
    let current_value: &mut i32 = state.entry(reg).or_insert(0);

    let test_predicate: fn(i32, i32) -> bool = match test_op {
        ">" => |a, b| a > b,
        ">=" => |a, b| a >= b,
        "==" => |a, b| a == b,
        "!=" => |a, b| a != b,
        "<" => |a, b| a < b,
        "<=" => |a, b| a <= b,
        other => panic!(format!("Unknown comparison operator: {}", other))
    };

    if test_predicate(current_test, test_cmp) {
        match op {
            "inc" => {
                *current_value = *current_value + diff;
            },
            "dec" => {
                *current_value = *current_value - diff;
            },
            _ => panic!("Invalid instruction: {}", op)
        };
    }
}

fn solve_a(input: &Vec<&str>) -> i32 {
    let mut state: HashMap<String, i32> = HashMap::new();
    for inst in input {
        execute_instruction(inst, &mut state);
    }

    *state.values().max().expect("No maximum found.")
}

fn solve_b(input: &Vec<&str>) -> i32 {
    let mut state: HashMap<String, i32> = HashMap::new();
    let mut max: i32 = 0;
    for inst in input {
        execute_instruction(inst, &mut state);
        max = ::std::cmp::max(max, *state.values().max().expect("No maximum found."));
    }

    max
}
