mod hash_object;
mod init;

use hash_object::hash_object;
use init::init;
pub enum Command {
    Init,
    HashBlob(String),
}

impl Command {
    pub fn execute(&self) {
        match self {
            Command::Init => {
                init();
            }
            Command::HashBlob(file_path) => {
                hash_object(file_path.to_string()).unwrap();
            }
        }
    }
}
