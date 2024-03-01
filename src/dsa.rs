use std::collections::HashMap;

pub struct Table {
    table: HashMap<String, i64>,
    current_var: i64,
}

impl Table {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            current_var: 0,
        }
    }

    pub fn acquire_num(self: &mut Self) -> i64 {
        self.current_var += 1;
        self.current_var
    }

    pub fn insert(self: &mut Self, id: &str, var: Option<i64>) -> i64 {
        let var = var.unwrap_or_else(|| self.acquire_num());
        self.table.insert(id.to_string(), var);
        var
    }

    pub fn get(self: &Self, id: &str) -> Option<i64> {
        self.table.get(id).map(|v| *v)
    }
}