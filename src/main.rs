use std::env;

mod solution_001;
mod solution_074;
mod solution_092;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "001" { println!("{}", solution_001::solve()); } // ~0.00 seconds real
        if args[1] == "074" { println!("{}", solution_074::solve(1000000, 60)); } // ~0.83 seconds real
        if args[1] == "092" { println!("{}", solution_092::solve(10000000, 89)); } // ~0.83 seconds real
    }
}
