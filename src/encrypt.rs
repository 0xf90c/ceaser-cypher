#[derive(Debug, Default)]
pub struct Encrypt {
    shift: i8,
    index: i8,
}

impl Encrypt {
    pub fn new() -> Encrypt {
        Encrypt { shift: 0, index: 0 }
    }

    fn encrypt(&self) {
        (self.shift + self.index) % 26;
    }
}
