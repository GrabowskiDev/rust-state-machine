mod automats;

struct DAS {
    alphabet: Alphabet,
    state_names: HashSet<String>,
    states: Vec<Node>,
    current_state: Node,
    start_state: Node,
}
// ε
impl Automat for DAS {
    fn add_state(&mut self, state: &Node){
        if self.state_names.contains(state.get_name()) {
            panic!("State with name {} already exists", state.get_name());
        }
        if state.get_name().is_empty() {
            panic!("State name cannot be empty");
        }

        connections = state.get_connections();
        for key in connections.keys() {
            if !self.alphabet.contains(key) {
                panic!("Character {} not in alphabet", key);
            }
        }

        for state in connections.values() {
            if !self.state_names.contains(state.get_name()) {
                panic!("State {} doesn't exist", state.get_name());
            }
        }

        self.states.push(state.clone());
    }

    fn add_state_name(&mut self, name: &str){
        if self.state_names.contains(name) {
            panic!("State with name {} already exists", name);
        }
        if name.is_empty() {
            panic!("State name cannot be empty");
        }
        self.state_names.insert(name.to_string());
    }
    
    fn set_start_node(&mut self, state: &Node){
        self.start_state = state.clone();
        self.current_state = state.clone();
    }
    
    fn get_current_state(&self) -> &Node{
        &self.current_state
    }

    fn process(&mut self, input: &str) -> bool{
        let mut current_state = self.get_current_state();
        for character in input.chars() {
            if let Some(next_state) = current_state.get_next_state(character) {
                current_state = next_state;
            } else {
                return false;
            }
        }
        current_state.is_accepting()
    }
}
