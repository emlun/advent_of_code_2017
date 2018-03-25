use std::collections::HashSet;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = String;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        (solve_a(&input), solve_b(&input))
    }
}

#[derive(Debug)]
struct RawNode {
    id: String,
    weight: u32,
    children: HashSet<String>,
}

fn parse_tree(input: &Vec<&str>) -> Vec<RawNode> {
    input
        .iter()
        .map(|line| {
            let mut arrow_split = line.split(" -> ");

            let node_part = arrow_split.next().expect(&format!("Invalid input line: {}", line));
            let children_part = arrow_split.next();

            let mut node_split = node_part.split(" ");
            let id = node_split.next().expect(&format!("Failed to parse node ID from line: {}", line));
            let weight: u32 = node_split.next()
                .and_then(|word|
                    word.chars()
                        .skip_while(|&c| c == '(')
                        .take_while(|&c| c != ')')
                        .collect::<String>()
                        .parse()
                        .ok()
                )
                .expect(&format!("Failed to parse weight from line: {}", line));

            let children: HashSet<String> = children_part
                .unwrap_or("")
                .split_terminator(", ")
                .map(String::from)
                .collect();

            RawNode {
                id: String::from(id),
                weight,
                children,
            }
        })
        .collect()
}

fn solve_a(input: &Vec<&str>) -> String {

    let nodes: Vec<RawNode> = parse_tree(input);

    let child_node_ids: HashSet<String> = nodes.iter()
        .flat_map(|node| node.children.iter())
        .cloned()
        .collect();

    let (_, mut roots): (Vec<RawNode>, Vec<RawNode>) = nodes
        .into_iter()
        .partition(|node| child_node_ids.contains(&node.id));

    if roots.len() != 1 {
        panic!("Expected exactly one root, found {}", roots.len());
    }

    let raw_root: RawNode = roots.pop().expect("Expected exactly one root, found none.");

    String::from(raw_root.id)
}

fn solve_b(input: &Vec<&str>) -> u32 {
    0
}
