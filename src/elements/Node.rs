mod elements;

use std::ascii::Char;
use std::collections::HashMap;

struct Node{
    name: str,
    connections: HashMap<char, Node>,
    accepting: bool,
}

impl Node {
    fn next(&self, character: &char) -> Node {
        &self.connections.get(&character)
    }

    fn get_connections(&self) -> HashMap<char, Node> {
        &self.connections
    }

    fn get_name(&self) -> str {
        &name
    }
}

// Node current_
// current_ = current_.next("d")