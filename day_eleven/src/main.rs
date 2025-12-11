use std::collections::HashMap;
use aoc_common as common;

fn count_paths(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == "out" {
        return 1;
    }
    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let mut total = 0;
    if let Some(children) = graph.get(node) {
        for child in children {
            total += count_paths(child, graph, memo);
        }
    }

    memo.insert(node.to_string(), total);
    total
}

fn count_paths_with_requirements(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let seen_dac = seen_dac || node == "dac";
    let seen_fft = seen_fft || node == "fft";

    let key = (node.to_string(), seen_dac, seen_fft);

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if node == "out" {
        let result = if seen_dac && seen_fft { 1 } else { 0 };
        memo.insert(key, result);
        return result;
    }

    let mut total = 0;
    if let Some(children) = graph.get(node) {
        for next in children {
            total += count_paths_with_requirements(next, graph, seen_dac, seen_fft, memo);
        }
    }

    memo.insert(key, total);
    total
}


fn main() {
    let lines = common::read_file_to_vec("input.txt").unwrap();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let (node, rest) = line.split_once(':').unwrap();
        let children = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        graph.insert(node.to_string(), children);
    }

    let mut memo = HashMap::new();
    let result = count_paths("you", &graph, &mut memo);
    println!("Valid paths from you = {}", result);
    
    let mut memo2 = HashMap::new();
    let result = count_paths_with_requirements("svr", &graph, false, false, &mut memo2);
    println!("Valid paths from svr = {}", result);
}
