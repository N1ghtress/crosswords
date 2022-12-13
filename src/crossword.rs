use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{ BufReader, prelude::* },
};
use rand::prelude::*;

const DEFAULT_NB_WORD: u8 = 10;
const DEFAULT_NB_TRY_POPULATE: usize = 10;
const DEFAULT_NB_TRY_ADD_WORD: usize = 20000;
const WORDS_FILE_PATH: &str = "./liste_mots_francais.txt";

pub struct Crossword {
    word_max_length: u8,
    nb_word: u8,
    words: Vec<String>,
    revealed_words: Vec<bool>
}

impl Crossword {
    pub fn new(word_max_length: u8, nb_word: u8) -> Crossword {
        Crossword {
            word_max_length,
            nb_word,
            words: Vec::new(),
            revealed_words: vec![false; nb_word as usize]
        }
    }

    pub fn populate_words(&mut self) {
        let mut populate_attempt: usize = 0;
        while self.words.is_empty() && populate_attempt < DEFAULT_NB_TRY_POPULATE {
            self.words = self.try_populate_words();
            populate_attempt += 1;
        }
    }

    fn try_populate_words(&mut self) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();

        // Selection of a word of maximum length using all different letters
        let mut french_words = self.get_words_from_file();
        french_words.retain(|s| s.len() <= self.word_max_length as usize && s.len() >= 3);

        let mut rng = rand::thread_rng();
        let mut word_index = rng.gen_range(0..french_words.len());
        
        let mut word = french_words[word_index].clone();
        
        while self.is_not_valid_first_word(&word.as_str()) {
            word_index = rng.gen_range(0..french_words.len());
            word = french_words[word_index].clone();
        }
        
        words.push(word.clone());
        let letters = word.as_bytes().into_iter().cloned().collect::<HashSet<_>>();
        let mut add_word_attempt: usize = 0;

        // Selection of word_number words of length <= maximum length and that use the same letter as word
        while (words.len() as u8) < self.nb_word && add_word_attempt < DEFAULT_NB_TRY_ADD_WORD {
            word_index = rng.gen_range(0..french_words.len());
            word = french_words[word_index].clone();
            if self.is_valid_word(&words, &word.as_str(), &letters) {
                words.push(word.clone());
            }
            add_word_attempt += 1;
        }

        if (words.len() as u8) < self.nb_word {
            words = Vec::new()
        }

        words
    }  

    fn get_words_from_file(&self) -> Vec<String> {
        let french_words_file = File::open(WORDS_FILE_PATH).unwrap();
        let mut br = BufReader::new(french_words_file);
        let mut french_words_string = String::new();
        br.read_to_string(&mut french_words_string).unwrap();
        
        french_words_string.split("\r\n").map(|s| s.to_string()).collect()
    }

    fn is_not_valid_first_word(&self, word: &str) -> bool {
        // word.as_bytes().into_iter().cloned().collect::<HashSet<_>>().len() != self.max_length as usize
        word.len() as u8 != self.word_max_length
    }

    fn is_valid_word(&self, words: &Vec<String>, word: &str, letters: &HashSet<u8>) -> bool {
        !words.contains(&word.to_string())
        && self.is_composed_of(&word, &letters)
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
        f.write_str("]\n")?;
        f.write_str(self.words.len().to_string().as_str())?;
        Ok(())
    }
}
