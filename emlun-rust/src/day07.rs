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

#[derive(Debug)]
struct Node {
    id: String,
    weight: u32,
    children: Vec<Node>,
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

fn assemble_tree(raw_nodes: Vec<RawNode>) -> Vec<Node> {
    let child_node_ids: HashSet<String> = raw_nodes.iter()
        .flat_map(|node| node.children.iter())
        .cloned()
        .collect();

    let (children, raw_roots): (Vec<&RawNode>, Vec<&RawNode>) = raw_nodes
        .iter()
        .partition(|node|
            child_node_ids.contains(&node.id)
        );

    println!("child_node_ids: {:?}", child_node_ids);
    println!("raw_roots: {:?}", raw_roots);

    assemble_subtree(raw_roots, &children)
}

fn assemble_subtree(roots: Vec<&RawNode>, rest: &Vec<&RawNode>) -> Vec<Node> {
    roots.iter()
        .map(|root| {
            let (children, rest): (Vec<&RawNode>, Vec<&RawNode>) = rest
                .iter()
                .partition(|child|
                    root.children.contains(&child.id)
                );

            let children = assemble_subtree(children, &rest);

            Node {
                id: root.id.clone(),
                weight: root.weight,
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

    let (children, mut roots): (Vec<RawNode>, Vec<RawNode>) = nodes
        .into_iter()
        .partition(|node| child_node_ids.contains(&node.id));

    if roots.len() != 1 {
        panic!("Expected exactly one root, found {}", roots.len());
    }

    let raw_root: RawNode = roots.pop().expect("Expected exactly one root, found none.");

    String::from(raw_root.id)
}

fn solve_b(input: &Vec<&str>) -> u32 {
    let raw_nodes = parse_tree(input);

    let roots = assemble_tree(raw_nodes);

    println!("roots: {:?}", roots.iter().map(|n| &n.id).collect::<Vec<&String>>());
    println!("root children: {:?}", roots.iter().flat_map(|r| &r.children).map(|n| &n.id).collect::<Vec<&String>>());

    0
}
