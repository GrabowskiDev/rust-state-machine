use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    name: String,
    connections: HashMap<char, String>,
    accepting: bool,
}

impl Node {
    pub fn new(name: &str, accepting: bool) -> Self {
        Self {
            name: name.to_string(),
            connections: HashMap::new(),
            accepting,
        }
    }

    pub fn add_connection(&mut self, symbol: char, state_name: &str) {
        self.connections.insert(symbol, state_name.to_string());
    }

    pub fn get_connections(&self) -> &HashMap<char, String> {
        &self.connections
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_accepting(&self) -> bool {
        self.accepting
    }
}