mod commands;

use commands::Command;
pub fn start(args: Vec<String>) {
    match args[1].as_str() {
        "init" => Command::Init.execute(),
        "hash-object" => Command::HashBlob(args).execute(),
        "cat-file" => Command::CatFile(args).execute(),
        _ => println!("Command not found"),
    }
}
