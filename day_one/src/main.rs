mod solutions;
mod utils;

fn main() {
    let file_path = "input.txt";

    let lines = match utils::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let part1_password = solutions::part_one(&lines);
    let part2_password = solutions::part_two(&lines);

    println!("Part 1 Password: {}", part1_password);
    println!("Part 2 Password: {}", part2_password);
}