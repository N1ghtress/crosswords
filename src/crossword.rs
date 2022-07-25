use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{ BufReader, prelude::* }
};
use rand::prelude::*;

const DEFAULT_NUMBER_OF_WORDS: usize = 10;

pub type Position = (usize, usize);

pub struct Crossword {
    words: Vec<String>,
    //positions: Vec<Position>,
}

impl Crossword {
    pub fn new(max_length: u8) -> Crossword {
        Crossword {
            words: {
                // Selection of a word of maximum length using all different letters
                let french_words_file = File::open("liste_mots_francais.txt").unwrap();
                let mut br = BufReader::new(french_words_file);
                let mut french_words_string = String::new();
                br.read_to_string(&mut french_words_string).unwrap();

                let french_words: Vec<String> = french_words_string.split("\r\n").map(|s| s.to_string()).collect();

                let mut rng = rand::thread_rng();
                let mut word_index = rng.gen_range(0..french_words.len());
                
                let mut word = french_words[word_index].clone();
                while word.len() != max_length as usize || Crossword::number_different_characters(word.as_str()) != max_length as usize {
                    word_index = rng.gen_range(0..french_words.len());
                    word = french_words[word_index].clone();
                }

                let mut words = HashSet::from([word.clone()]);
                let letters = word.as_bytes().into_iter().cloned().collect::<HashSet<_>>();

                // Selection of 9 words of length <= maximum length and that use the same letter as word
                while words.len() != DEFAULT_NUMBER_OF_WORDS {
                    word_index = rng.gen_range(0..french_words.len());
                    word = french_words[word_index].clone();
                    if word.len() <= max_length as usize
                    && Crossword::contains_letters(&word, &letters)
                    && !words.contains(&word) {
                        words.insert(word);
                    }
                }

                words.into_iter().collect()
            },
            /*
            positions: {
                // Positionnement des mots dans une matrice virtuelle
            }
            */
        }
    }

    fn number_different_characters(str: &str) -> usize {
        str.as_bytes().into_iter().cloned().collect::<HashSet<_>>().len()
    }

    fn contains_letters(word: &str, letters: &HashSet<u8>) -> bool {
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