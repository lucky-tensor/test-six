use std::path::PathBuf;

use structopt::StructOpt;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use libra_management::error::Error;

use crate::storage_helper::StorageHelper;

#[derive(Debug, StructOpt)]
pub struct Init {
    #[structopt(long, short)]
    pub namespace: String,
    #[structopt(long, short)]
    pub path: Option<PathBuf>,
}


impl Init {
    pub fn execute(self) -> Result<String, Error> {
        
        let mut rl = Editor::<()>::new();

        println!("Enter your 0L mnemonic");

        let readline = rl.readline(">> ");

        match readline {
            Ok(mnemonic_string) => {
                let path: PathBuf;
                if self.path.is_some() {
                    path = self.path.unwrap();
                } else { 
                    path = PathBuf::from("~/.0L/node");
                }
                let helper = StorageHelper::new_with_path(path.into());
                helper.initialize_with_mnemonic(self.namespace.clone(), mnemonic_string);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                std::process::exit(-1);

            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                std::process::exit(-1);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                std::process::exit(-1);

            }
        }

        Ok("Keys Generated".to_string())
    }
}
