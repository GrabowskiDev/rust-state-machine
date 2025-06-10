use crate::elements::Alphabet::Alphabet;
use crate::elements::Node::Node;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct ENAS {
    pub(crate) alphabet: Alphabet,
    states: HashMap<String, ENASNode>,
    start_state: String,
}

#[derive(Clone)]
pub struct ENASNode {
    name: String,
    connections: HashMap<char, Vec<String>>, // char -> lista stanów docelowych
    accepting: bool,
}

impl ENASNode {
    pub fn new(name: &str, accepting: bool) -> Self {
        Self {
            name: name.to_string(),
            connections: HashMap::new(),
            accepting,
        }
    }

    pub fn add_connection(&mut self, symbol: char, state_names: Vec<String>) {
        self.connections.insert(symbol, state_names);
    }

    pub fn get_connections(&self) -> &HashMap<char, Vec<String>> {
        &self.connections
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_accepting(&self) -> bool {
        self.accepting
    }
}

impl ENAS {
    pub fn new(alphabet: Alphabet) -> Self {
        Self {
            alphabet,
            states: HashMap::new(),
            start_state: String::new(),
        }
    }

    pub fn add_state(&mut self, node: ENASNode) {
        let name = node.get_name().to_string();
        self.states.insert(name, node);
    }

    pub fn set_start_state(&mut self, name: &str) {
        self.start_state = name.to_string();
    }

    /// Przetwarzanie wejścia przez ENAS (NFA z epsilon-przejściami)
    pub fn process(&self, input: &str) -> bool {
        let mut current_states = self.epsilon_closure(&[self.start_state.clone()]);
        for c in input.chars() {
            let mut next_states = HashSet::new();
            for state in &current_states {
                if let Some(node) = self.states.get(state) {
                    if let Some(targets) = node.get_connections().get(&c) {
                        for target in targets {
                            next_states.insert(target.clone());
                        }
                    }
                }
            }
            current_states = self.epsilon_closure(&next_states.into_iter().collect::<Vec<_>>());
        }
        current_states.iter().any(|s| self.states.get(s).map_or(false, |n| n.is_accepting()))
    }

    /// Zwraca domknięcie epsilon dla podanych stanów
    fn epsilon_closure(&self, states: &[String]) -> HashSet<String> {
        let mut closure: HashSet<String> = states.iter().cloned().collect();
        let mut queue: VecDeque<String> = states.iter().cloned().collect();
        while let Some(state) = queue.pop_front() {
            if let Some(node) = self.states.get(&state) {
                if let Some(epsilon_targets) = node.get_connections().get(&'ε') {
                    for target in epsilon_targets {
                        if closure.insert(target.clone()) {
                            queue.push_back(target.clone());
                        }
                    }
                }
            }
        }
        closure
    }

    /// Walidacja ENAS: sprawdza tylko, czy wszystkie połączenia prowadzą do istniejących stanów
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let state_names: HashSet<_> = self.states.keys().cloned().collect();

        for (state, node) in &self.states {
            for (symbol, targets) in node.get_connections() {
                let symbol_str = if *symbol == 'ε' { "ε".to_string() } else { symbol.to_string() };
                for target in targets {
                    if !state_names.contains(target) {
                        errors.push(format!(
                            "Stan '{}' ma połączenie dla '{}' do nieistniejącego stanu '{}'.",
                            state,
                            symbol_str,
                            target
                        ));
                    }
                }
            }
        }
        errors
    }
}

