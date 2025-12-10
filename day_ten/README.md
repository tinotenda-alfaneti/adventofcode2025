# Day Ten: Button Press Optimization

**Problem Summary**
- **Goal (Part 1):** Given a machine with indicator lights (binary on/off) and buttons that toggle specific lights via XOR, find the minimum number of button presses to reach a target light configuration. Each machine has a target state shown as `[.##.]` (where `#` = on, `.` = off) and buttons like `(1,3)` that toggle lights at indices 1 and 3.
- **Goal (Part 2):** Given a machine with joltage counters (non-negative integers) and buttons that increment specific counters, find the minimum number of button presses to reach target counter values. Each machine has target values like `{3,5,4,7}` and buttons like `(0,2)` that increment counters at indices 0 and 2.

**Repository Files (relevant)**
- `src/lib.rs` — module declarations for the two parts.
- `src/indicator_presses.rs` — Part 1 solver (brute-force over XOR combinations).
- `src/joltage_presses.rs` — Part 2 solver (Integer Linear Programming).
- `src/main.rs` — runner that reads `input.txt` and prints results.
- `example.txt` — small example (3 machines).
- `input.txt` — full puzzle input.
- `tests/functional.rs` — tests for both parts.

## Part 1 — Indicator Lights (XOR toggle problem)

**Approach**
- Each button toggles a subset of indicator lights using XOR (0 → 1, 1 → 0).
- The order of button presses doesn't matter (XOR is commutative and associative).
- Pressing the same button twice cancels out (a XOR a = 0), so each button is either pressed once or not at all.
- This reduces the problem to: which subset of buttons, when XORed together, produces the target state?

**Implementation**
- Parse the target lights as a binary vector and each button as a binary vector indicating which lights it toggles.
- Try all 2^n button combinations (where n = number of buttons) using bitmask enumeration.
- For each combination, XOR the corresponding button vectors and check if the result equals the target.
- Track the minimum number of buttons pressed.

**Pseudocode**
```
function min_presses(machine):
    best = infinity
    for each subset of buttons (via bitmask):
        state = [0, 0, ..., 0]  // all lights off
        presses = 0
        for each button in subset:
            state = state XOR button_pattern
            presses += 1
        if state == target:
            best = min(best, presses)
    return best
```

**Complexity (Part 1)**
- Time: O(2^n × m) where n = number of buttons, m = number of lights per machine
- Space: O(m) for state vectors
- This is feasible because n is typically small (≤ 10-15 buttons per machine)

**Why brute force works here**
- The problem size is small enough (exponential in number of buttons, not lights).
- For 10 buttons, 2^10 = 1024 combinations is trivial to check.
- More sophisticated approaches (Gaussian elimination over GF(2)) exist but aren't necessary.

---

## Part 2 — Joltage Counters (Integer Linear Programming)

**Problem Statement**
- Each machine has k counters with target values `[t₀, t₁, ..., tₖ₋₁]`.
- Each button increments a specific subset of counters by 1.
- We need to find the minimum total button presses such that each counter reaches exactly its target value.

**Why Integer Linear Programming (ILP)?**
This is a classic **optimization problem** that can be formulated as an ILP:
- **Variables:** xᵢ = number of times button i is pressed (must be non-negative integer)
- **Objective:** Minimize Σ xᵢ (total presses)
- **Constraints:** For each counter j, Σ (xᵢ for buttons that affect counter j) = target_j

**Mathematical Formulation**
```
Minimize:    x₀ + x₁ + x₂ + ... + xₙ₋₁
Subject to:  For each counter j:
               Σ xᵢ (where button i affects counter j) = target_j
             xᵢ ≥ 0 and xᵢ ∈ ℤ (non-negative integers)
```

**Example Walkthrough**
Consider the first example machine:
```
Target: {3, 5, 4, 7}  (4 counters)
Buttons: (3), (1,3), (2), (2,3), (0,2), (0,1)
```

Variables: x₀, x₁, x₂, x₃, x₄, x₅ (times each button is pressed)

Constraints:
- Counter 0: x₄ + x₅ = 3  (buttons 4 and 5 affect counter 0)
- Counter 1: x₁ + x₅ = 5  (buttons 1 and 5 affect counter 1)
- Counter 2: x₂ + x₃ + x₄ = 4  (buttons 2, 3, 4 affect counter 2)
- Counter 3: x₀ + x₁ + x₃ = 7  (buttons 0, 1, 3 affect counter 3)

Objective: Minimize x₀ + x₁ + x₂ + x₃ + x₄ + x₅

The ILP solver finds the optimal solution (e.g., x₀=2, x₁=3, x₂=0, x₃=2, x₄=1, x₅=2 gives total = 10).

**Implementation Details**

The code uses the `good_lp` crate with the `microlp` solver:

```rust
// 1. Create decision variables (one per button)
let mut vars_problem = ProblemVariables::new();
let mut vars = vec![];
for _ in 0..num_buttons {
    let v = vars_problem.add(variable().min(0).integer());
    vars.push(v);
}

// 2. Define objective: minimize sum of all button presses
let objective: Expression = vars.iter().fold(Expression::from(0.0), |acc, &v| acc + v);

// 3. Create model
let mut model = vars_problem.minimise(objective).using(microlp);

// 4. Add constraints (one per counter)
for counter_idx in 0..num_counters {
    let target_val = machine.target[counter_idx] as f64;
    
    let mut constraint_expr = Expression::from(0.0);
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        if button.contains(&counter_idx) {
            constraint_expr += vars[button_idx];
        }
    }
    
    model = model.with(constraint!(constraint_expr == target_val));
}

// 5. Solve
let solution = model.solve()?;
```

**Why ILP instead of brute force?**
- Part 1 can use brute force because buttons toggle (on/off decision).
- Part 2 requires counting: each button can be pressed 0, 1, 2, ... times.
- If max target value is T and we have n buttons, brute force would be O(T^n) - exponentially worse!
- ILP efficiently explores the solution space using mathematical optimization techniques.

**How ILP solvers work (simplified)**
1. **Relaxation:** First solve as a continuous Linear Program (LP) ignoring integer constraints - this is fast (polynomial time).
2. **Branch and Bound:** If the LP solution has fractional values:
   - Pick a fractional variable (e.g., x₃ = 2.7)
   - Create two sub-problems: one with x₃ ≤ 2, one with x₃ ≥ 3
   - Recursively solve both, pruning branches that can't improve the best solution found
3. **Cutting Planes:** Add additional constraints to tighten the LP relaxation, reducing the search space

The `microlp` solver is a pure-Rust implementation of the simplex method with branch-and-bound.

**Complexity (Part 2)**
- Time: O(poly(n, k) × 2^d) where n = buttons, k = counters, d = problem depth
  - Polynomial in best case (when LP relaxation gives integer solution)
  - Exponential worst case (but with effective pruning in practice)
- Space: O(n × k) for constraint matrix
- Much faster than brute force for practical inputs

**Alternative approaches**
- **Greedy heuristics:** Often produce suboptimal solutions for this problem.
- **Dynamic Programming:** Possible but state space is large (all counter combinations).
- **Constraint Programming:** Similar to ILP but with different search strategies.
- **Other ILP solvers:** CBC, GLPK, or commercial solvers like Gurobi/CPLEX (faster but with dependencies).

---

## Rust-Specific Learnings

**Libraries used**
- `good_lp`: A Rust library for linear programming with multiple backend solvers
  - Provides high-level API for defining variables, objectives, and constraints
  - `microlp`: Pure-Rust LP solver (no external dependencies)
  - Alternative backends: CBC, HiGHS (require C++ libraries)

**Key Rust idioms**
- **Expression building with fold:**
  ```rust
  let objective = vars.iter().fold(Expression::from(0.0), |acc, &v| acc + v);
  ```
  Accumulates variable references into a single expression for the objective function.

- **Operator overloading:**
  ```rust
  constraint_expr += vars[button_idx];  // Expression implements AddAssign
  ```
  The `good_lp` library overloads `+`, `+=`, `==` for building mathematical expressions.
