use memoize::memoize;
use std::collections::{BTreeMap, HashSet};

use helpers::*;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let network = construct_graph(input);
    let you_index = network.nodes["you"];
    let out_index = network.nodes["out"];
    get_path_count(network, you_index, out_index)
}

fn part_2(input: &[String]) -> i64 {
    let network = construct_graph(input);

    // Get all indices
    let svr_index = network.nodes["svr"];
    let fft_index = network.nodes["fft"];
    let dac_index = network.nodes["dac"];
    let out_index = network.nodes["out"];

    // Two cases
    // svr -> fft -> dac -> out
    let svr_to_fft = get_path_count(network.clone(), svr_index, fft_index);
    let fft_to_dac = get_path_count(network.clone(), fft_index, dac_index);
    let dac_to_out = get_path_count(network.clone(), dac_index, out_index);
    let svr_fft_dac_out = svr_to_fft * fft_to_dac * dac_to_out;

    // svr -> dac -> fft -> out
    let svr_to_dac = get_path_count(network.clone(), svr_index, dac_index);
    let dac_to_fft = get_path_count(network.clone(), dac_index, fft_index);
    let fft_to_out = get_path_count(network.clone(), fft_index, out_index);
    let svr_dac_fft_out = svr_to_dac * dac_to_fft * fft_to_out;

    // Total path count
    let path_count = svr_fft_dac_out + svr_dac_fft_out;

    path_count
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Graph {
    nodes: BTreeMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

fn construct_graph(input: &[String]) -> Graph {
    let mut nodes: BTreeMap<String, usize> = BTreeMap::new();
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
    nodes: &mut BTreeMap<String, usize>,
    edges: &mut Vec<Vec<usize>>,
    seen_nodes: &mut HashSet<String>,
) {
    if !seen_nodes.contains(node) {
        seen_nodes.insert(node.clone());
        let index = nodes.len();
        nodes.insert(node.clone(), index);
        edges.push(Vec::new());
    }
}

#[memoize]
fn get_path_count(network: Graph, start: usize, end: usize) -> i64 {
    let mut path_count = 0;
    let neighbors = &network.edges[start];
    for neighbor in neighbors {
        if *neighbor == end {
            return 1;
        } else {
            path_count += get_path_count(network.clone(), *neighbor, end)
        }
    }

    path_count
}
