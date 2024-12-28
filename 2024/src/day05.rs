use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn create_graph(patterns: &[(i32, i32)]) -> DiGraph<i32, ()> {
    let mut graph = DiGraph::new();
    let mut node_indices = HashMap::new();

    // Create nodes for unique numbers
    for &(from, to) in patterns {
        if !node_indices.contains_key(&from) {
            let idx = graph.add_node(from);
            node_indices.insert(from, idx);
        }
        if !node_indices.contains_key(&to) {
            let idx = graph.add_node(to);
            node_indices.insert(to, idx);
        }
    }

    // Add edges
    for &(from, to) in patterns {
        let from_idx = node_indices[&from];
        let to_idx = node_indices[&to];
        graph.add_edge(from_idx, to_idx, ());
    }

    graph
}

fn parse_pattern(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return None;
    }

    let from = parts[0].trim().parse().ok()?;
    let to = parts[1].trim().parse().ok()?;
    Some((from, to))
}

fn sort_nodes(graph: &DiGraph<i32, ()>) -> Vec<NodeIndex> {
    match toposort(&graph, None) {
        Ok(sorted_nodes) => sorted_nodes,
        Err(_) => {
            println!("Could not sort topologically, sorting by in-degree");
            let mut nodes: Vec<_> = graph
                .node_indices()
                .map(|idx| {
                    let in_degree = graph
                        .neighbors_directed(idx, petgraph::Direction::Incoming)
                        .count();
                    let value = graph[idx];
                    (in_degree, value, idx)
                })
                .collect();

            nodes.sort_by(|&(deg_a, val_a, _), &(deg_b, val_b, _)| {
                deg_a.cmp(&deg_b).then(val_a.cmp(&val_b))
            });

            nodes.into_iter().map(|(_, _, idx)| idx).collect()
        }
    }
}

fn find_path_between_nodes(
    graph: &DiGraph<i32, ()>,
    start: NodeIndex,
    end: NodeIndex,
    allowed_nodes: &HashSet<NodeIndex>,
) -> Option<Vec<NodeIndex>> {
    let mut visited = HashSet::new();
    let mut paths = VecDeque::new();
    paths.push_back(vec![start]);
    visited.insert(start);
    while let Some(path) = paths.pop_front() {
        let current = *path.last().unwrap();
        if current == end {
            return Some(path);
        }

        for edge in graph.edges(current) {
            let next = edge.target();
            if !visited.contains(&next) && allowed_nodes.contains(&next) {
                visited.insert(next);
                let mut new_path = path.clone();
                new_path.push(next);
                paths.push_back(new_path);
            }
        }
    }
    None
}

fn find_ordered_path(graph: &DiGraph<i32, ()>, values: &[i32]) -> Option<Vec<NodeIndex>> {
    // Create mapping from value to NodeIndex
    let value_to_idx: HashMap<i32, NodeIndex> =
        graph.node_indices().map(|idx| (graph[idx], idx)).collect();

    // Convert values to node indices
    let nodes: Vec<NodeIndex> = values
        .iter()
        .filter_map(|v| value_to_idx.get(v))
        .copied()
        .collect();

    if nodes.len() != values.len() {
        return None; // Some values don't exist in graph
    }

    let allowed_nodes: HashSet<NodeIndex> = nodes.iter().copied().collect();
    let mut final_path = vec![nodes[0]];

    // Find path between each consecutive pair
    for window in nodes.windows(2) {
        let from = window[0];
        let to = window[1];
        match find_path_between_nodes(graph, from, to, &allowed_nodes) {
            Some(path) => {
                final_path.extend(&path[1..]);
            }
            None => return None,
        }
    }

    Some(final_path)
}

pub fn solve_part1() {
    let patterns = match open_file("data/05/rules.txt") {
        Ok(reader) => reader
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| parse_pattern(&line))
            .collect::<Vec<(i32, i32)>>(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let updates: Vec<Vec<i32>> = match open_file("data/05/updates.txt") {
        Ok(reader) => reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                line.split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect()
            })
            .collect(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let graph = create_graph(&patterns);
    let _sorted_nodes = sort_nodes(&graph);
    let mut total = 0;
    for update in updates {
        match find_ordered_path(&graph, &update) {
            Some(path) => {
                // Floor division to get the middle index
                let middle = path.len() / 2;
                total += update[middle];
                // println!("Found the middle value: {}", update[middle]);
            }
            None => {} //println!("No path exists visiting values in this order")},
        }
    }
    println!("Total: {}", total);
}

pub fn solve_part2() {}
