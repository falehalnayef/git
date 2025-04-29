mod cat_file;
mod hash_object;
mod init;

use cat_file::cat_file;
use hash_object::hash_object;
use init::init;
pub enum Command {
    Init,
    HashBlob(Vec<String>),
    CatFile(Vec<String>),
}

impl Command {
    pub fn execute(&self) {
        match self {
            Command::Init => {
                init();
            }
            Command::HashBlob(args) => {
                if let Err(e) = hash_object(args.clone()) {
                    println!("{}", e);
                }
            }

            Command::CatFile(args) => {
                if let Err(e) = cat_file(args.clone()) {
                    println!("{}", e);
                }
            }
        }
    }
}
