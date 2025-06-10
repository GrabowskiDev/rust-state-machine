use std::collections::HashSet;

pub struct Alphabet {
    alphabet: HashSet<char>,
}

impl Alphabet {
    pub fn new() -> Self {
        Self { alphabet: HashSet::new() }
    }

    pub fn add(&mut self, character: char) {
        self.alphabet.insert(character);
    }

    pub fn contains(&self, character: &char) -> bool {
        self.alphabet.contains(character)
    }

    pub fn len(&self) -> usize {
        self.alphabet.len()
    }
}