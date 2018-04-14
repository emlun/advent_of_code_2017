pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = i32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(input), solve_b(input))
    }
}

struct State {
    depth: u32,
    in_garbage: bool,
    ignore_next: bool,
    score: u32,
}
impl State {
    fn new() -> State {
        State {
            depth: 0,
            in_garbage: false,
            ignore_next: false,
            score: 0,
        }
    }
}

fn step(state: &mut State, next: char) {
    if state.in_garbage {
        if state.ignore_next {
            state.ignore_next = false;
        } else {
            match next {
                '!' => { state.ignore_next = true; }
                '>' => { state.in_garbage = false; }
                _   => {}
            }
        }
    } else {
        match next {
            '{' => { state.depth += 1; }
            '}' => {
                state.score += state.depth;
                state.depth -= 1;
            }
            '<' => { state.in_garbage = true; }
            ',' => {}
            _   => panic!(format!("Unknown non-garbage: {}", next))
        }
    }
}

fn solve_a(input: &Vec<&str>) -> u32 {
    let mut state = State::new();

    input.iter()
        .flat_map(|line| line.chars())
        .for_each(|next| step(&mut state, next));

    state.score
}

fn solve_b(input: &Vec<&str>) -> i32 {
    0
}
