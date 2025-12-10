use good_lp::*;

#[derive(Debug)]
struct Machine {
    target: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

fn parse_machine(line: &str) -> Machine {
    let start = line.rfind('{').expect("missing {");
    let end = line.rfind('}').expect("missing }");
    let joltage_text = &line[start + 1..end];

    let target = joltage_text
        .split(',')
        .map(|x| x.trim().parse::<u32>().expect("bad number"))
        .collect::<Vec<_>>();

    let mut buttons = vec![];
    for chunk in line.split('(').skip(1) {
        let inside = chunk.split(')').next().unwrap();
        let indices = inside
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();
        buttons.push(indices);
    }

    Machine { target, buttons }
}

fn min_presses_ilp(machine: &Machine) -> u32 {
    let num_counters = machine.target.len();
    let num_buttons = machine.buttons.len();

    let mut vars_problem = ProblemVariables::new();
    let mut vars = vec![];
    for _ in 0..num_buttons {
        let v = vars_problem.add(variable().min(0).integer());
        vars.push(v);
    }

    let objective: Expression = vars.iter().fold(Expression::from(0.0), |acc, &v| acc + v);

    let mut model = vars_problem.minimise(objective).using(microlp);

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

    let solution = model.solve();

    match solution {
        Ok(sol) => {
            let mut total = 0;
            for var in vars.iter().take(num_buttons) {
                total += sol.value(*var).round() as u32;
            }
            total
        }
        Err(e) => {
            panic!("No solution found for machine: {:?}", e);
        }
    }
}

pub fn solve(lines: &[String]) -> u32 {
    let machines = lines.iter().map(|l| parse_machine(l)).collect::<Vec<_>>();
    machines.iter().map(min_presses_ilp).sum()
}
