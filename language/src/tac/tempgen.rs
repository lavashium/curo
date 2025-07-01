use crate::TacVal;

pub struct TempGen {
    counter: usize,
}

impl TempGen {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn next(&mut self) -> TacVal {
        let temp = format!("@t{}", self.counter);
        self.counter += 1;
        TacVal::Var(temp)
    }
}
