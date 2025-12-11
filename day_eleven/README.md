# Day Eleven: Path Counting in Directed Graphs

**Problem Summary**
- **Goal (Part 1):** Given a directed graph represented as adjacency lists, count all valid paths from a starting node `"you"` to a destination node `"out"`.
- **Goal (Part 2):** Given a directed graph, count all valid paths from a starting node `"svr"` to `"out"` that pass through **both** specific required intermediate nodes: `"dac"` and `"fft"` (in any order).

**Repository Files (relevant)**
- `src/lib.rs` — module declarations and shared graph creation logic.
- `src/day_to_out.rs` — Part 1 solver (simple path counting with memoization).
- `src/svr_to_out.rs` — Part 2 solver (path counting with visitation requirements).
- `src/main.rs` — runner that reads `input.txt` and prints results.
- `example_p1.txt` — small example for Part 1 (10 lines).
- `example_p2.txt` — small example for Part 2 (13 lines).
- `input.txt` — full puzzle input.
- `tests/functional.rs` — tests for both parts.

## Part 1 — Simple Path Counting

**Approach**
- Count all paths from `"you"` to `"out"` in a directed graph.
- Each node can have multiple outgoing edges (children).
- Use dynamic programming with memoization to avoid recomputing paths from the same node.

**Implementation**
- Parse the input to build an adjacency list representation of the graph.
- Use recursive depth-first search (DFS) with memoization:
  - Base case: if current node is `"out"`, return 1 (found one valid path).
  - Recursive case: sum the paths from all children of the current node.
  - Cache results to avoid redundant computation.

**Pseudocode**
```
function count_paths(node, graph, memo):
    if node == "out":
        return 1
    
    if node in memo:
        return memo[node]
    
    total = 0
    for each child of node in graph:
        total += count_paths(child, graph, memo)
    
    memo[node] = total
    return total
```

**Example Walkthrough (Part 1)**
```
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
```

Starting from `"you"`:
- Path 1: you → bbb → ddd → ggg → out
- Path 2: you → bbb → eee → out
- Path 3: you → ccc → ddd → ggg → out
- Path 4: you → ccc → eee → out
- Path 5: you → ccc → fff → out

**Total: 5 paths**

**Complexity (Part 1)**
- Time: O(V + E) where V = number of nodes, E = number of edges (with memoization, each node is processed once)
- Space: O(V) for memoization cache and recursion stack
- Without memoization, this could be exponential in graphs with many paths

**Why memoization works here**
- The graph structure allows for overlapping subproblems (multiple paths may converge at the same node).
- Once we know how many paths exist from node X to "out", we don't need to recompute it.
- This transforms an exponential brute-force approach into a linear-time solution.

---

## Part 2 — Path Counting with Required Nodes

**Problem Statement**
- Count all valid paths from `"svr"` to `"out"` that pass through **both** `"dac"` AND `"fft"`.
- The order doesn't matter (can visit dac then fft, or fft then dac).
- A path is only valid if it visits both required nodes before reaching `"out"`.

**Why State-Based Dynamic Programming?**
This is a classic **constrained path counting problem** that requires tracking:
- Current position in the graph
- Whether we've seen `"dac"` yet
- Whether we've seen `"fft"` yet

**State Space**
- **State:** (current_node, seen_dac, seen_fft)
- **Transitions:** When moving to a child node, update the seen flags if the child is "dac" or "fft"
- **Goal:** Reach `"out"` with both flags set to true

**Mathematical Formulation**
```
State: (node, has_dac, has_fft)

Transitions:
  - If node == "dac", set has_dac = true
  - If node == "fft", set has_fft = true
  - Recurse to all children with updated state

Base cases:
  - If node == "out" and has_dac and has_fft: return 1
  - If node == "out" and !(has_dac and has_fft): return 0
  - Otherwise: sum paths from all children
```

**Example Walkthrough (Part 2)**
```
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
```

Starting from `"svr"`, we need paths that visit both `"dac"` and `"fft"`:
- svr → aaa → fft → ccc → eee → dac → fff → ggg → out (visits fft then dac) ✓
- svr → aaa → fft → ccc → eee → dac → fff → hhh → out (visits fft then dac) ✓
- svr → aaa → fft → ccc → ddd → hub → fff → ... (visits fft but not dac) ✗
- svr → bbb → tty → ccc → eee → dac → fff → ggg → out (visits dac but not fft) ✗
- etc.

The solution counts only paths satisfying the constraint.

**Implementation Details**

The code uses memoization with a composite key `(node, seen_dac, seen_fft)`:

```rust
fn count_paths_with_requirements(
    start_node: &str,
    graph: &HashMap<String, Vec<String>>,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    // Update flags if we're at dac or fft
    let seen_dac = seen_dac || start_node == "dac";
    let seen_fft = seen_fft || start_node == "fft";

    let key = (start_node.to_string(), seen_dac, seen_fft);

    // Check cache
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // Base case: reached destination
    if start_node == "out" {
        let result = if seen_dac && seen_fft { 1 } else { 0 };
        memo.insert(key, result);
        return result;
    }

    // Recursive case: sum paths from children
    let mut total = 0;
    if let Some(children) = graph.get(start_node) {
        for next in children {
            total += count_paths_with_requirements(
                next, graph, seen_dac, seen_fft, memo
            );
        }
    }

    memo.insert(key, total);
    total
}
```

**Key Insights**
1. **State tracking:** The boolean flags track which required nodes we've visited so far.
2. **Memoization key:** Must include both the node and the boolean state to avoid incorrect caching.
3. **Flag propagation:** Once a flag is set to true, it remains true for all descendants in that path.
4. **Base case validation:** Only count paths that have visited all required nodes.

**Complexity (Part 2)**
- Time: O((V + E) × 2²) = O(4V + 4E) where the factor of 4 comes from the 2² possible boolean states
- Space: O(4V) for memoization cache with state tuples
- Still linear in the size of the graph, just with a constant factor increase

**Why this approach is optimal**
- Avoids generating all possible paths explicitly (which could be exponential).
- Uses memoization to share computation across paths with the same constraint state.
- The state space is small (only 4 states per node: neither, dac only, fft only, both).

---

## Input Format

Both parts use the same input format:
```
node1: child1 child2 child3
node2: child4 child5
node3: child6
...
```

Each line defines a node and its outgoing edges (space-separated list of children).

**Parsing Logic** (`create_graph` in `lib.rs`):
```rust
fn create_graph(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
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
```

---

## Testing

Run tests with:
```bash
cargo test
```

Tests verify:
- Part 1: Example yields 5 paths from "you" to "out"
- Part 2: Example yields correct count of paths visiting both "dac" and "fft"

Run the full solution:
```bash
cargo run --release
```

---

## Key Takeaways

1. **Graph path counting** is a common dynamic programming problem that benefits greatly from memoization.
2. **State-augmented DP** allows us to track constraints (like "must visit certain nodes") without enumerating all paths.
3. **Recursive DFS with memoization** is often clearer and more maintainable than iterative approaches for these problems.
4. **Time complexity** transforms from exponential (naïve) to linear (with proper memoization) in the graph size.
