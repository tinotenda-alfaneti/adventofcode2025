use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) -> usize {
    let graph = super::create_graph(lines);
    let mut memo = HashMap::new();
    count_paths("you", &graph, &mut memo)
}

fn count_paths(
    start_node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if start_node == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(start_node) {
        return cached;
    }

    let mut total = 0;
    if let Some(children) = graph.get(start_node) {
        for child in children {
            total += count_paths(child, graph, memo);
        }
    }

    memo.insert(start_node.to_string(), total);
    total
}
