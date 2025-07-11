use accessors::accessors;
use std::collections::HashMap;

#[accessors]
#[derive(Clone)]
pub struct TempGen {
    temp_counter: usize,
    label_counter: usize,
    loop_label_counters: HashMap<String, usize>,
}

impl TempGen {
    pub fn new() -> Self {
        Self { 
            temp_counter:        0,
            label_counter:       0,
            loop_label_counters: HashMap::new(),
        }
    }

    pub fn temp(&mut self) -> String {
        let temp = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    pub fn temp_from(&mut self, prefix: String) -> String {
        let temp = format!("{}.{}", prefix, self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    pub fn label(&mut self, label: impl ToString) -> String {
        let temp = format!("_{}_{}", label.to_string(), self.label_counter);
        self.label_counter += 1;
        temp
    }

    pub fn loop_label(&mut self, user_label: &str, kind: &str) -> String {
        let count = self
            .loop_label_counters
            .entry(user_label.to_string())
            .or_insert_with(|| {
                let c = self.label_counter;
                self.label_counter += 1;
                c
            });

        format!(".L{}_{}_{}", kind, user_label, count)
    }
}
