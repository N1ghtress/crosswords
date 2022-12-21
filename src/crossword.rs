use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{ BufReader, prelude::*, self },
};
use rand::prelude::*;

const POPULATE_ATTEMPS: usize = 10;
const ADD_WORD_ATTEMPS: usize = 20000;
const WORDS_FILE_PATH: &str = "pli07.txt";

pub struct Crossword {
    word_max_length: usize,
    nb_word: usize,
    letters: Vec<char>,
    words: Vec<String>,
    found_words: Vec<bool>,
    revealed_letters: Vec<char>
    // observers: Vec<Observer>
}

impl Crossword {
    pub fn new(word_max_length: usize, nb_word: usize) -> Crossword {
        Crossword {
            word_max_length,
            nb_word,
            letters: Vec::new(),
            words: Vec::new(),
            found_words: vec![false; nb_word],
            revealed_letters: Vec::new()
        }
    }

    pub fn populate_words(&mut self) {
        let mut populate_attempt: usize = 0;

        let mut french_words = self.get_words_from_file();
        french_words.retain(|s| s.len() <= self.word_max_length && s.len() >= 3);

        while self.words.is_empty() && populate_attempt < POPULATE_ATTEMPS {
            self.try_populate_words(&french_words);
            populate_attempt += 1;
        }
    }

    pub fn start(&mut self) {
        while self.found_words.contains(&false) {
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

    fn try_populate_words(&mut self, dict: &Vec<String>) {
        self.words = Vec::new();

        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(0..dict.len());
        
        while dict[index].chars().collect::<HashSet<_>>().len() != self.word_max_length {
            index = rng.gen_range(0..dict.len());
        }
        
        self.words.push(dict[index].clone());
        self.letters = dict[index].chars().collect::<HashSet<_>>().into_iter().collect();
        
        let mut attempts: usize = 0;
        while self.words.len() < self.nb_word && attempts < ADD_WORD_ATTEMPS {
            index = rng.gen_range(0..dict.len());
            let word = dict[index].clone();
            if !self.words.contains(&word.to_string()) && self.is_composed_of(&word, &self.letters) {
                self.words.push(word.clone());
            }
            attempts += 1;
        }
    }  

    fn get_words_from_file(&self) -> Vec<String> {
        let french_words_file = File::open(WORDS_FILE_PATH).unwrap();
        let mut br = BufReader::new(french_words_file);
        let mut french_words_string = String::new();
        br.read_to_string(&mut french_words_string).unwrap();
        
        french_words_string.split("\n").map(|s| s.to_string()).collect()
    }

    fn is_composed_of(&self, word: &str, letters: &Vec<char>) -> bool {
        for letter in word.chars() {
            if !letters.contains(&letter) {
                return false
            }
        }
    
        true
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
            self.found_words[index] = true;
        }
    }
}

impl Display for Crossword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = self.words.clone();
        words.into_iter().enumerate().for_each(|(i, word)| {
            if self.found_words[i] {
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
