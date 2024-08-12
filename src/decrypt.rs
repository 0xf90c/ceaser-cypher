#[derive(Debug, Default)]
pub struct Decrypt {
    shift: i8,
    index: i8,
}

impl Decrypt {
    pub fn new() -> Decrypt {
        Decrypt { shift: 0, index: 0 }
    }

    fn decrypt(&self) {
        (self.index + 26 - self.shift) % 26;
    }
}
