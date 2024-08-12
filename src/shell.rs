#![allow(while_true)]

use super::decrypt::Decrypt;
use super::encrypt::Encrypt;

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Welcome,

    // Selection stage of action
    Selection,

    // ..

    // Exit stage
    Exit,
}

#[derive(Debug, Default)]
pub struct Shell {
    stage: Mode,
    encrypt: Encrypt,
    decrypt: Decrypt,
    inputs: Vec<String>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            stage: Mode::Welcome,
            encrypt: Encrypt::new(),
            decrypt: Decrypt::new(),
            inputs: Vec::new(),
        }
    }

    fn welcome(&self) {}

    fn selection(&self) {}

    fn exit(&self) {
        println!("Exiting...");
        std::process::exit(0);
    }

    pub fn interactive(&self) {
        while true {
            match self.stage {
                Mode::Welcome => self.welcome(),
                Mode::Selection => self.selection(),
                Mode::Exit => self.exit(),
            }
        }
    }
}
