use util::countable::Countable;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solver {}
impl ::framework::Solver for Solver {
    type A = String;
    type B = u32;
    fn solve(&self, input: &Vec<&str>) -> (Self::A, Self::B) {
        let raw_nodes = parse_tree(input);

        let roots: Vec<Node> = assemble_tree(raw_nodes);

        if roots.len() != 1 {
            panic!("Expected exactly one root, found {}", roots.len());
        }

        let root = roots
            .into_iter()
            .next()
            .expect("Expected exactly one root, found none.");

        (solve_a(&root), solve_b(&root))
    }
}

#[derive(Debug)]
struct RawNode {
    weight: u32,
    children: HashSet<String>,
}

#[derive(Debug)]
struct Node {
    id: String,
    weight: u32,
    children: Vec<Node>,
}
impl Node {
    fn total_weight(&self) -> u32 {
        self.weight
            + self.children
                .iter()
                .map(Node::total_weight)
                .fold(0, |sum, child_weight| sum + child_weight)
    }
}

fn parse_tree(input: &Vec<&str>) -> HashMap<String, RawNode> {
    input
        .iter()
        .map(|line| {
            let mut arrow_split = line.split(" -> ");

            let node_part = arrow_split.next().expect(&format!("Invalid input line: {}", line));
            let children_part = arrow_split.next();

            let mut node_split = node_part.split(" ");
            let id: String = String::from(
                node_split.next()
                    .expect(&format!("Failed to parse node ID from line: {}", line))
            );
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

            (id,
                RawNode {
                    weight,
                    children,
                }
            )
        })
        .collect()
}

fn assemble_tree(raw_nodes: HashMap<String, RawNode>) -> Vec<Node> {
    let child_node_ids: HashSet<String> = raw_nodes.values()
        .flat_map(|node| node.children.iter())
        .cloned()
        .collect();

    let (mut children, raw_roots): (HashMap<String, RawNode>, HashMap<String, RawNode>) = raw_nodes
        .into_iter()
        .partition(|&(ref id, _)|
            child_node_ids.contains(id)
        );

    raw_roots.into_iter()
        .map(|(id, root)|
            assemble_subtree(id, root, &mut children)
        )
        .collect()
}

fn assemble_subtree(root_id: String, root: RawNode, children: &mut HashMap<String, RawNode>) -> Node {
    let children: Vec<Node> = root.children.into_iter()
        .map(|child_id| {
            let child = children.remove(&child_id).expect(&format!("Child disappeared: {}", &child_id));
            assemble_subtree(child_id, child, children)
        })
        .collect();

    Node {
        id: root_id,
        weight: root.weight,
        children,
    }
}

fn solve_a(root: &Node) -> String {
    root.id.clone()
}

fn solve_b(root: &Node) -> u32 {
    if let FinalResult(_, correct_weight) = find_unbalanced_node(root) {
        correct_weight
    } else {
        panic!("Bad node not found.");
    }
}

enum FindUnbalancedResult<'a> {
    ParentOf(&'a Node),
    FinalResult(&'a Node, u32),
}
use self::FindUnbalancedResult::ParentOf;
use self::FindUnbalancedResult::FinalResult;

fn find_unbalanced_node(root: &Node) -> FindUnbalancedResult {
    match root.children.len() {
        2 => panic!(
                    "Expected no node to have exactly 2 children, but {} does: {:?}",
                    root.id,
                    root.children.iter().map(|c| &c.id).collect::<Vec<&String>>()
            ),
        1 => find_unbalanced_node(&root.children[0]),
        _ => {
            let child_weights: Vec<u32> = root.children.iter()
                .map(Node::total_weight)
                .collect();

            let weight_counts: HashMap<&u32, u32> = child_weights.iter().counts();

            let (good_weights, bad_weights): (Vec<(usize, &u32)>, Vec<(usize, &u32)>) = child_weights.iter()
                .enumerate()
                .partition(|&(_, w)| *weight_counts.get(w).unwrap_or(&0) > 1);

            match bad_weights.first() {
                None => ParentOf(&root),
                Some(&(i, _)) => {
                    let bad_node = &root.children[i];

                    let prelim_result = find_unbalanced_node(bad_node);

                    let good_weight = *good_weights
                        .first()
                        .expect("No correct weight found.")
                        .1;

                    finish_result(root, good_weight, prelim_result)
                }
            }
        }
    }
}

fn finish_result<'a>(
    parent: &'a Node,
    correct_total_weight: u32,
    result: FindUnbalancedResult<'a>
) -> FindUnbalancedResult<'a> {
    match result {
        ParentOf(bad_node) => {
            let total_children_weight = bad_node.children.iter()
                .map(Node::total_weight)
                .fold(0, |sum, w| sum + w);

            FinalResult(
                parent,
                correct_total_weight - total_children_weight
            )
        }
        FinalResult(_, _) => result
    }
}
