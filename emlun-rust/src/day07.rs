use std::collections::HashMap;
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

    let roots: Vec<Node> = assemble_tree(raw_nodes);
    let root = Node {
        id: String::from("root"),
        weight: 0,
        children: roots,
    };

    println!("roots: {:?}", root.children.iter().map(|n| &n.id).collect::<Vec<&String>>());
    println!("root children: {:?}", root.children.iter().flat_map(|r| &r.children).map(|n| &n.id).collect::<Vec<&String>>());

    println!("root weight: {}", compute_weight(&root));

    if let FinalResult(bad_node, correct_weight) = find_unbalanced_node(&root) {
        println!("bad node: {}", bad_node.id);
        println!("correct weight: {}", correct_weight);

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
                .map(compute_weight)
                .collect();

            let weight_counts: HashMap<u32, u32> = child_weights.iter()
                .fold(
                    HashMap::new(),
                    |mut map, &w| {
                        let current_count = *map.entry(w).or_insert(0);
                        map.insert(w, current_count + 1);
                        map
                    }
                );

            let bad_weight = child_weights.iter()
                .enumerate()
                .find(|&(_, w)| weight_counts[w] == 1);

            match bad_weight {
                None => ParentOf(&root),
                Some((i, _)) => {
                    let bad_node = &root.children[i];

                    let prelim_result = find_unbalanced_node(bad_node);

                    finish_result(root, &weight_counts, prelim_result)

                    // let correct_total_weight = weight_counts.iter()
                        // .find(|&(_, &count)| count > 1)
                        // .map(|(w, _)| w)
                        // .unwrap();

                    // let total_children_weight = bad_node.children.iter()
                        // .map(compute_weight)
                        // .fold(0, |sum, w| sum + w);

                    // FinalResult(
                        // &root.children[i],
                        // correct_total_weight - total_children_weight
                    // )
                }
            }
        }
    }
}

fn finish_result<'a>(
    parent: &'a Node,
    weight_counts: &HashMap<u32, u32>,
    result: FindUnbalancedResult<'a>
) -> FindUnbalancedResult<'a> {
    match result {
        ParentOf(bad_node) => {
            let correct_total_weight = weight_counts.iter()
                .find(|&(_, &count)| count > 1)
                .map(|(w, _)| w)
                .unwrap();

            let total_children_weight = bad_node.children.iter()
                .map(compute_weight)
                .fold(0, |sum, w| sum + w);

            FinalResult(
                parent,
                correct_total_weight - total_children_weight
            )
        }
        FinalResult(_, _) => result
    }
}

fn compute_weight(node: &Node) -> u32 {
    node.weight
        + node.children
            .iter()
            .map(compute_weight)
            .fold(0, |sum, child_weight| sum + child_weight)
}
