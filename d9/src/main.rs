use d9::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let n = solve_p1(input.as_str());
    println!("Part 1: Number of unique positions {}", n);
    let n = solve_p2(input.as_str());
    println!("Part 2: Number of unique positions {}", n);
}
