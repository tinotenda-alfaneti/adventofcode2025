use std::collections::HashMap;

pub mod day_to_out;
pub mod svr_to_out;

pub fn create_graph(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let (node, rest) = line.split_once(':').unwrap();
        let children = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        graph.insert(node.to_string(), children);
    }
    graph
}
