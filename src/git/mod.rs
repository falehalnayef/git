mod commands;

use commands::Command;
pub fn start() {
    let command = Command::Init;
    command.execute();
}
