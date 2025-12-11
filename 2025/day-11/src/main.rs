use std::collections::{HashMap, HashSet};

use helpers::*;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let network = construct_graph(input);

    // Perform modified DFS starting from "you" node to "out" node
    let mut stack = Vec::new();
    let mut path_count = 0;
    stack.push(network.nodes["you"]);
    while let Some(current_node) = stack.pop() {
        let neighbors = &network.edges[current_node];
        for neighbor in neighbors {
            // If the neighbor is "out", increment path count instead of adding to stack
            if neighbor == &network.nodes["out"] {
                path_count += 1;
            } else {
                stack.push(*neighbor);
            }
        }
    }

    path_count
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

fn construct_graph(input: &[String]) -> Graph {
    let mut nodes: HashMap<String, usize> = HashMap::new();
    let mut edges = Vec::new();
    let mut seen_nodes: HashSet<String> = HashSet::new();

    for line in input {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();

        check_and_insert_node(&node, &mut nodes, &mut edges, &mut seen_nodes);

        let neighbors: Vec<String> = parts[1].split(" ").map(|s| s.to_string()).collect();
        neighbors.iter().for_each(|neighbor| {
            check_and_insert_node(neighbor, &mut nodes, &mut edges, &mut seen_nodes);
            let node_index = nodes[&node];
            let neighbor_index = nodes[neighbor];
            edges[node_index].push(neighbor_index);
        });
    }

    Graph { nodes, edges }
}

fn check_and_insert_node(
    node: &String,
    nodes: &mut HashMap<String, usize>,
    edges: &mut Vec<Vec<usize>>,
    seen_nodes: &mut HashSet<String>,
) {
    if !seen_nodes.contains(node) {
        seen_nodes.insert(node.clone());
        nodes.insert(node.clone(), nodes.len());
        edges.push(Vec::new());
    }
}
