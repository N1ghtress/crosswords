use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{ BufReader, prelude::*, self },
};
use rand::prelude::*;

const DEFAULT_NB_TRY_POPULATE: usize = 10;
const DEFAULT_NB_TRY_ADD_WORD: usize = 20000;
const WORDS_FILE_PATH: &str = "./liste_mots_francais.txt";

pub struct Crossword {
    word_max_length: u8,
    nb_word: u8,
    letters: Vec<char>,
    words: Vec<String>,
    found_words: Vec<u8>,
    revealed_letters: Vec<char>
    // observers: Vec<Observer>
}

impl Crossword {
    pub fn new(word_max_length: u8, nb_word: u8) -> Crossword {
        Crossword {
            word_max_length,
            nb_word,
            letters: Vec::new(),
            words: Vec::new(),
            found_words: Vec::new(),
            revealed_letters: Vec::new()
        }
    }

    pub fn populate_words(&mut self) {
        let mut populate_attempt: usize = 0;
        while self.words.is_empty() && populate_attempt < DEFAULT_NB_TRY_POPULATE {
            self.words = self.try_populate_words();
            populate_attempt += 1;
        }
    }

    pub fn start(&mut self) {
        while self.found_words.len() != self.words.len() {
            println!("{self}"); // notify vue
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(_n) => {
                    buffer = String::from(buffer.trim().to_ascii_uppercase());
                    self.process_input(&mut buffer);
                },
                Err(error) => panic!("{error}"),
            }
        }
    }

    fn try_populate_words(&mut self) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();

        let mut french_words = self.get_words_from_file();
        french_words.retain(|s| s.len() <= self.word_max_length as usize && s.len() >= 3);

        let mut rng = rand::thread_rng();
        let mut word_index = rng.gen_range(0..french_words.len());
        
        let mut word = french_words[word_index].clone();
        
        while word.len() as u8 != self.word_max_length {
            word_index = rng.gen_range(0..french_words.len());
            word = french_words[word_index].clone();
        }
        
        words.push(word.clone());
        // word.as_bytes().into_iter().cloned().collect::<HashSet<u8>>();
        self.letters = word.chars().collect::<HashSet<char>>().into_iter().collect::<Vec<char>>();
        let mut add_word_attempt: usize = 0;

        // Selection of word_number words of length <= maximum length and that use the same letter as word
        while (words.len() as u8) < self.nb_word && add_word_attempt < DEFAULT_NB_TRY_ADD_WORD {
            word_index = rng.gen_range(0..french_words.len());
            word = french_words[word_index].clone();
            if !words.contains(&word.to_string()) && self.is_composed_of(&word, &self.letters) {
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

    fn is_composed_of(&self, word: &str, letters: &Vec<char>) -> bool {
        for letter in word.chars() {
            if !letters.contains(&letter) {
                return false;
            }
        }
    
        return true
    }

    fn process_input(&mut self, input: &mut String) {
        let input = input.trim().to_ascii_uppercase();
        if input == "!SOLUTION" {
            self.revealed_letters.append(&mut self.letters.clone());
        }
        if input == "!HINT" {
            let mut rng = rand::thread_rng();
            let letter_index = rng.gen_range(0..self.letters.len());
            self.revealed_letters.push(self.letters.remove(letter_index));
        }
        if self.words.contains(&input.to_string()) {
            let index = self.words.iter().position(|r| r == &input.to_string()).unwrap();
            self.found_words.push(index as u8);
        }
    }
}

impl Display for Crossword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = self.words.clone();
        words.into_iter().enumerate().for_each(|(i, word)| {
            if self.found_words.contains(&(i as u8)) {
                f.write_str(&word).unwrap();
            } else {
                for char in word.chars() {
                    if self.revealed_letters.contains(&char) {
                        f.write_str(&format!("{char}")).unwrap();
                    } else {
                        f.write_str("_").unwrap();
                    }
                }
            }
            f.write_str(" ").unwrap();
        });

        f.write_str("\nAvailable letters: [ ").unwrap();
        for letter in &self.letters {
            f.write_str(&format!("{} ", *letter)).unwrap();
        }
        f.write_str("]\n").unwrap();
        Ok(())
    }
}
