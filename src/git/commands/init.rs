use std::fs;

pub fn init() {
    fs::create_dir(".gut").unwrap();
    fs::create_dir(".gut/objects").unwrap();
    fs::create_dir(".gut/refs").unwrap();
    fs::write(".gut/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized gut directory");
}
