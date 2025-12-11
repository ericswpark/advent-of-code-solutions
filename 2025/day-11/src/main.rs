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
    let network = construct_graph(input);

    // Perform modified DFS starting from "svr" node to "out" node
    let mut stack = Vec::new();
    let mut path_count = 0;
    stack.push(vec![String::from("svr")]);
    while let Some(current_path) = stack.pop() {
        let current_node = current_path.last().unwrap();
        let current_node_index = network.nodes[current_node];
        let neighbors = &network.edges[current_node_index];
        for neighbor_index in neighbors {
            // If the neighbor is "out", and we pass through both "fft" and "dac"
            if neighbor_index == &network.nodes["out"]
                && current_path.contains(&String::from("fft"))
                && current_path.contains(&String::from("dac"))
            {
                // This path is valid
                path_count += 1;
            } else {
                // Add neighbor to path chain and push to stack
                let neighbor = &network.node_indices[neighbor_index];
                let mut new_path = current_path.clone();
                new_path.push(neighbor.clone());
                stack.push(new_path);
            }
        }
    }

    path_count
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, usize>,
    node_indices: HashMap<usize, String>,
    edges: Vec<Vec<usize>>,
}

fn construct_graph(input: &[String]) -> Graph {
    let mut nodes: HashMap<String, usize> = HashMap::new();
    let mut node_indices: HashMap<usize, String> = HashMap::new();
    let mut edges = Vec::new();
    let mut seen_nodes: HashSet<String> = HashSet::new();

    for line in input {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();

        check_and_insert_node(
            &node,
            &mut nodes,
            &mut node_indices,
            &mut edges,
            &mut seen_nodes,
        );

        let neighbors: Vec<String> = parts[1].split(" ").map(|s| s.to_string()).collect();
        neighbors.iter().for_each(|neighbor| {
            check_and_insert_node(
                neighbor,
                &mut nodes,
                &mut node_indices,
                &mut edges,
                &mut seen_nodes,
            );
            let node_index = nodes[&node];
            let neighbor_index = nodes[neighbor];
            edges[node_index].push(neighbor_index);
        });
    }

    Graph {
        nodes,
        node_indices,
        edges,
    }
}

fn check_and_insert_node(
    node: &String,
    nodes: &mut HashMap<String, usize>,
    node_indices: &mut HashMap<usize, String>,
    edges: &mut Vec<Vec<usize>>,
    seen_nodes: &mut HashSet<String>,
) {
    if !seen_nodes.contains(node) {
        seen_nodes.insert(node.clone());
        let index = nodes.len();
        nodes.insert(node.clone(), index);
        node_indices.insert(index, node.clone());
        edges.push(Vec::new());
    }
}
