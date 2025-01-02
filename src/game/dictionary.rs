use rand::seq::IteratorRandom;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct Dictionary(HashMap<usize, HashSet<String>>);

impl Dictionary {
    pub fn new() -> Self {
        let contents = fs::read_to_string("src/words.json").unwrap();
        let words: Vec<String> = serde_json::from_str(&contents).unwrap();
        let mut hashmap: HashMap<usize, HashSet<String>> = HashMap::new();
        for word in words.into_iter() {
            match word.len() {
                4 => hashmap.entry(4).or_default().insert(word.to_lowercase()),
                5 => hashmap.entry(5).or_default().insert(word.to_lowercase()),
                6 => hashmap.entry(6).or_default().insert(word.to_lowercase()),
                _otherwise => false,
            };
        }
        Dictionary(hashmap)
    }

    pub fn is_valid_word(&self, word: &str) -> bool {
        self.0
            .get(&word.len())
            .is_some_and(|words| words.contains(&word.to_lowercase()))
    }

    pub fn get_random_word(&self, word_length: usize) -> Option<String> {
        match self.0.get(&word_length) {
            None => None,
            Some(word_list) => {
                let mut rng = rand::thread_rng();
                word_list.iter().choose(&mut rng).cloned()
            }
        }
    }
}
