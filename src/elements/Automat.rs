mod elements;

pub trait Automat {
    fn add_state_name(&mut self, name: &str);
    fn add_state(&mut self, state: &Node);
    fn set_start_node(&mut self, state: &Node);
    fn get_current_state(&self) -> &Node;
    fn process(&mut self, input: &str) -> bool{

    }
}