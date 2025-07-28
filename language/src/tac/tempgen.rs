use accessors::accessors;
use std::collections::HashMap;
use zawarudo::zawarudo;

#[accessors]
#[derive(Clone)]
pub struct TempGen {
    temp_counter: usize,
    temp_prefix_counter: HashMap<String, usize>,
    label_counter: usize,
    loop_label_counters: HashMap<String, usize>,
    loop_label_ids: HashMap<String, usize>
}

impl TempGen {
    pub fn new() -> Self {
        Self { 
            temp_counter:        0,
            temp_prefix_counter: HashMap::new(),
            label_counter:       0,
            loop_label_counters: HashMap::new(),
            loop_label_ids:      HashMap::new(),
        }
    }

    #[zawarudo(label = "TempGen")]
    pub fn temp(&mut self) -> String {
        let temp = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    #[zawarudo(label = "TempGen")]
    pub fn temp_from(&mut self, prefix: String) -> String {
        let count = self.temp_prefix_counter.entry(prefix.clone()).or_insert(0);
        let temp = format!("{}.{}", prefix, count);
        *count += 1;
        temp
    }

    #[zawarudo(label = "TempGen")]
    pub fn label(&mut self, label: impl ToString) -> String {
        let temp = format!("_lbl_{}_{}", label.to_string(), self.label_counter);
        self.label_counter += 1;
        temp
    }

    #[zawarudo(label = "TempGen")]
    pub fn loop_label(&mut self, user_label: &str, kind: &str) -> String {
        let id = *self.loop_label_ids.entry(user_label.to_string()).or_insert_with(|| {
            let counter = self.loop_label_counters.entry(user_label.to_string()).or_insert(0);
            let current = *counter;
            *counter += 1;
            current
        });

        format!("_{}_{}_{}", kind, user_label, id)
    }
}