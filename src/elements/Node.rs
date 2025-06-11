use std::collections::HashMap;

// Część wspólna dla wszystkich węzłów
#[derive(Clone)]
pub struct NodeCommon {
    name: String,
    accepting: bool,
}

impl NodeCommon {
    pub fn new(name: &str, accepting: bool) -> Self {
        Self {
            name: name.to_string(),
            accepting,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_accepting(&self) -> bool {
        self.accepting
    }
}

// Trait z metodami wspólnymi
pub trait NodeBase {
    fn get_name(&self) -> &str;
    fn is_accepting(&self) -> bool;
}

// DASNode – deterministyczny
#[derive(Clone)]
pub struct DASNode {
    common: NodeCommon,
    connections: HashMap<char, String>,
}

impl DASNode {
    pub fn new(name: &str, accepting: bool) -> Self {
        Self {
            common: NodeCommon::new(name, accepting),
            connections: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, symbol: char, state_name: &str) {
        self.connections.insert(symbol, state_name.to_string());
    }

    pub fn get_connections(&self) -> &HashMap<char, String> {
        &self.connections
    }
}

impl NodeBase for DASNode {
    fn get_name(&self) -> &str {
        self.common.get_name()
    }
    fn is_accepting(&self) -> bool {
        self.common.is_accepting()
    }
}

// ENASNode – niedeterministyczny z epsilon
#[derive(Clone)]
pub struct ENASNode {
    common: NodeCommon,
    connections: HashMap<char, Vec<String>>,
}

impl ENASNode {
    pub fn new(name: &str, accepting: bool) -> Self {
        Self {
            common: NodeCommon::new(name, accepting),
            connections: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, symbol: char, state_names: Vec<String>) {
        self.connections.insert(symbol, state_names);
    }

    pub fn get_connections(&self) -> &HashMap<char, Vec<String>> {
        &self.connections
    }
}

impl NodeBase for ENASNode {
    fn get_name(&self) -> &str {
        self.common.get_name()
    }
    fn is_accepting(&self) -> bool {
        self.common.is_accepting()
    }
}