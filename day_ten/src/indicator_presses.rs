use std::cmp::min;

#[derive(Debug)]
struct Machine {
    target: Vec<u8>,
    buttons: Vec<Vec<u8>>,
}

fn xor_vec(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

fn parse_machine(line: &str) -> Machine {
    let light_part = line.split(']').next().unwrap().trim_start_matches('[');
    let target = light_part
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();

    let mut buttons = vec![];
    for chunk in line.split('(').skip(1) {
        let button_str = chunk.split(')').next().unwrap();
        let indices = button_str
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let mut button_vec = vec![0; target.len()];
        for idx in indices {
            button_vec[idx] = 1;
        }
        buttons.push(button_vec);
    }

    Machine { target, buttons }
}

fn min_presses(machine: &Machine) -> usize {
    let n = machine.buttons.len();
    let mut best = usize::MAX;

    for mask in 0..(1 << n) {
        let mut state = vec![0; machine.target.len()];
        let mut presses = 0;

        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                state = xor_vec(&state, &machine.buttons[i]);
                presses += 1;
            }
        }

        if state == machine.target {
            best = min(best, presses);
        }
    }

    best
}

pub fn solve(lines: &[String]) -> usize {
    let machines = lines.iter().map(|l| parse_machine(l)).collect::<Vec<_>>();
    let total: usize = machines.iter().map(min_presses).sum();

    total
}
