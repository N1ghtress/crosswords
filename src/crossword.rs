use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{ BufReader, prelude::* },
};
use rand::prelude::*;

const DEFAULT_NUMBER_OF_WORDS: usize = 10;
const WORDS_FILE_PATH: &str = "./liste_mots_francais.txt";

pub type Position = (usize, usize);

pub struct Crossword {
    max_length: usize,
    words: Vec<String>,
    positions: Vec<Position>
}

impl Crossword {
    pub fn new(max_length: usize) -> Crossword {
        Crossword {
            max_length,
            words: Vec::new(),
            positions: Vec::new()
        }
    }

    pub fn populate_words(&mut self) {
        // Selection of a word of maximum length using all different letters
        let mut french_words = self.get_words_from_file();
        french_words.retain(|s| s.len() <= self.max_length && s.len() >= 3);

        let mut rng = rand::thread_rng();
        let mut word_index = rng.gen_range(0..french_words.len());
        
        let mut word = french_words[word_index].clone();
        
        while self.is_not_valid_first_word(&word.as_str()) {
            word_index = rng.gen_range(0..french_words.len());
            word = french_words[word_index].clone();
        }
        
        self.words.push(word.clone());
        let letters = word.as_bytes().into_iter().cloned().collect::<HashSet<_>>();

        // Selection of 9 words of length <= maximum length and that use the same letter as word
        while self.words_is_not_full() {
            word_index = rng.gen_range(0..french_words.len());
            word = french_words[word_index].clone();
            if self.is_valid_word(&word.as_str(), &letters) {
                self.words.push(word.clone());
            }
        }
    }

    pub fn place_words(&mut self) {
        self.positions.push((0, 0));


    }    

    fn get_words_from_file(&self) -> Vec<String> {
        let french_words_file = File::open(WORDS_FILE_PATH).unwrap();
        let mut br = BufReader::new(french_words_file);
        let mut french_words_string = String::new();
        br.read_to_string(&mut french_words_string).unwrap();
        
        french_words_string.split("\r\n").map(|s| s.to_string()).collect()
    }

    fn is_not_valid_first_word(&self, word: &str) -> bool {
        word.as_bytes().into_iter().cloned().collect::<HashSet<_>>().len() != self.max_length
    }

    fn is_valid_word(&self, word: &str, letters: &HashSet<u8>) -> bool {
        !self.words.contains(&word.to_string())
        && self.is_composed_of(&word, &letters)
    }

    fn words_is_not_full(&self) -> bool {
        self.words.len() != DEFAULT_NUMBER_OF_WORDS
    }

    fn is_composed_of(&self, word: &str, letters: &HashSet<u8>) -> bool {
        for letter in word.as_bytes() {
            if !letters.contains(letter) {
                return false;
            }
        }
    
        return true
    }
}

impl Display for Crossword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[ ")?;
        for word in &self.words {
            f.write_str(word.as_str())?;
            f.write_str(" ")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}
