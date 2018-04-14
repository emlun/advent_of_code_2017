pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = u32;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {

        let mut state = State::new();

        input.iter()
            .flat_map(|line| line.chars())
            .for_each(|next| step(&mut state, next));

        (state.score, state.garbage_count)
    }
}

struct State {
    depth: u32,
    garbage_count: u32,
    ignore_next: bool,
    in_garbage: bool,
    score: u32,
}
impl State {
    fn new() -> State {
        State {
            depth: 0,
            garbage_count: 0,
            ignore_next: false,
            in_garbage: false,
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
                _   => { state.garbage_count += 1; }
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
