use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) -> usize {
    let graph = super::create_graph(lines);
    let mut memo = HashMap::new();
    count_paths_with_requirements("svr", &graph, false, false, &mut memo)
}

fn count_paths_with_requirements(
    start_node: &str,
    graph: &HashMap<String, Vec<String>>,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let seen_dac = seen_dac || start_node == "dac";
    let seen_fft = seen_fft || start_node == "fft";

    let key = (start_node.to_string(), seen_dac, seen_fft);

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if start_node == "out" {
        let result = if seen_dac && seen_fft { 1 } else { 0 };
        memo.insert(key, result);
        return result;
    }

    let mut total = 0;
    if let Some(children) = graph.get(start_node) {
        for next in children {
            total += count_paths_with_requirements(next, graph, seen_dac, seen_fft, memo);
        }
    }

    memo.insert(key, total);
    total
}
