use std::collections::HashSet;

mod elements;

struct Alphabet {
    alphabet: HashSet<char>,
    length: usize
}

impl Alphabet {
    fn add(&self, character: char){
        &self.alphabet.insert(character);
    }

    fn contains(&self, character: char) -> bool {
        self.alphabet.contains(&character)
    }

    fn get_length(&self) -> usize {
        alphabet.len()
    }
}