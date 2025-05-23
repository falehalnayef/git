mod git;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No command provided");
    } else {
        git::start(args);
    }
}
