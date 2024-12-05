pub struct Processor {
    on: bool,
}


impl Processor {
    pub fn new() -> Self {
        Self { on: true }
    }


    pub fn command_mul(&mut self, a: i64, b: i64) -> i64 {
        if self.on {
            return a * b;
        }
        else {
            return 0;
        }
    }


    pub fn command_do(&mut self) {
        self.on = true;
    }


    pub fn command_dont(&mut self) {
        self.on = false;
    }
}
