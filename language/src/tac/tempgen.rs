use accessors::accessors;

#[accessors]
#[derive(Clone)]
pub struct TempGen {
    temp_counter: usize,
    label_counter: usize,
}

impl TempGen {
    pub fn new() -> Self {
        Self { 
            temp_counter:  0,
            label_counter: 0,
        }
    }

    pub fn temp(&mut self) -> String {
        let temp = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    pub fn label(&mut self, label: impl ToString) -> String {
        let temp = format!("{}{}", label.to_string(), self.label_counter);
        self.label_counter += 1;
        temp
    }
}
