mod commands;

use commands::Command;
pub fn start() {
    //    let command = Command::Init;
    //  command.execute();
    let command2 = Command::HashBlob("hello.txt".to_string());

    command2.execute();
}
