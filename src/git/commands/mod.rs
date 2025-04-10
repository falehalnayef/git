mod init;
use init::init;

pub enum Command {
    Init,
}

impl Command {
    pub fn execute(&self) {
        match self {
            Command::Init => {
                init();
            }
        }
    }
}
