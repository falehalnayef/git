use std::fs;

pub fn init() {
    fs::create_dir(".gut").expect("Failed to create .gut folder.");
    fs::create_dir(".gut/objects").expect("Failed to create .gut/objects folder.");
    fs::create_dir(".gut/refs").expect("Failed to create .gut/refs folder.");
    fs::write(".gut/HEAD", "ref: refs/heads/main\n").expect("Failed to create .gut/HEAD flie.");
    println!("Initialized gut directory");
}
