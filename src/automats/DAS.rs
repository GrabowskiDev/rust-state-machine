use crate::elements::Alphabet::Alphabet;
use crate::elements::Node::Node;
use std::collections::{HashMap, HashSet};

pub struct DAS {
    pub(crate) alphabet: Alphabet,
    states: HashMap<String, Node>,
    start_state: String,
}

impl DAS {
    pub fn new(alphabet: Alphabet) -> Self {
        Self {
            alphabet,
            states: HashMap::new(),
            start_state: String::new(),
        }
    }

    pub fn add_state(&mut self, node: Node) {
        let name = node.get_name().to_string();
        self.states.insert(name, node);
    }

    pub fn set_start_state(&mut self, name: &str) {
        self.start_state = name.to_string();
    }

    pub fn process(&self, input: &str) -> bool {
        let mut current = self.start_state.clone();
        for c in input.chars() {
            let node = match self.states.get(&current) {
                Some(n) => n,
                None => return false,
            };
            if let Some(next) = node.get_connections().get(&c) {
                current = next.clone();
            } else {
                return false;
            }
        }
        self.states.get(&current).map_or(false, |n| n.is_accepting())
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let state_names: HashSet<_> = self.states.keys().cloned().collect();
        let alphabet_vec: Vec<char> = self.alphabet.alphabet.iter().cloned().collect();

        for (state, node) in &self.states {
            for (_symbol, target) in node.get_connections() {
                if !state_names.contains(target) {
                    errors.push(format!(
                        "Stan '{}' ma połączenie do nieistniejącego stanu '{}'.",
                        state, target
                    ));
                }
            }
        }

        for (state, node) in &self.states {
            for &symbol in &alphabet_vec {
                if !node.get_connections().contains_key(&symbol) {
                    errors.push(format!(
                        "Stan '{}' nie ma połączenia dla znaku '{}'.",
                        state, symbol
                    ));
                }
            }
        }

        errors
    }
}
