extern crate rand;

use rand::{thread_rng, Rng};

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use crate::constants::{UNKNOWN_WORDS_PATH, ALLOWED_WORDS_PATH};

pub struct WordBank {
    unknown_words: Vec<String>,
    allowed_words: Vec<String>,
}

impl WordBank {
    pub fn init() -> Result<WordBank, io::Error> {
        let file = File::open(UNKNOWN_WORDS_PATH)?;
        let reader = BufReader::new(file);
        
        let mut unknown_words: Vec<String> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            unknown_words.push(line);
        }
        
        let file = File::open(ALLOWED_WORDS_PATH)?;
        let reader = BufReader::new(file);
        
        let mut allowed_words: Vec<String> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            allowed_words.push(line);
        }

        Ok(WordBank { unknown_words, allowed_words })
    }

    pub fn get_unknown_words(&self) -> &Vec<String> {
        &self.unknown_words
    }

    pub fn get_allowed_words(&self) -> &Vec<String> {
        &self.allowed_words
    }

    pub fn get_random_word_in_unknown_words(&self) -> String {
        let mut rng = thread_rng();
        let unknown_words_len = self.unknown_words.len();

        assert_ne!(0, unknown_words_len);

        let rand_index = rng.gen_range(0..self.unknown_words.len());

        self.unknown_words[rand_index].clone()
    }

    pub fn in_unknown_words(&self, target_word: &str) -> bool {
        self.unknown_words
            .binary_search(&target_word.to_string())
            .is_ok()
    }

    pub fn in_allowed_words(&self, target_word: &str) -> bool {
        self.allowed_words
            .binary_search(&target_word.to_string())
            .is_ok()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_in_unknown_words() {
        let wordbank = WordBank::init().unwrap();

        let correct_words: [&str; 5] = [
            "aback",
            "zonal",
            "vigor",
            "linen",
            "crass",
        ];

        for word in correct_words {
            assert!(wordbank.in_unknown_words(word));
        }

        let incorrect_words: [&str; 5] = [
            " ",
            "",
            "he",
            "hell",
            "mowwasdasasd"
        ];

        for word in incorrect_words {
            assert!(!wordbank.in_unknown_words(word));
        }
    }

    #[test]
    fn test_in_allowed_words() {    
        let wordbank = WordBank::init().unwrap();
    
        let correct_words: [&str; 4] = [
            // "aahed",
            "zymic",
            "incel",
            "roque",
            "longa",
        ];
    
        for word in correct_words {
            assert!(wordbank.in_allowed_words(word));
        }
    
        let incorrect_words: [&str; 5] = [
            " ",
            "",
            "he",
            "hell",
            "mowwasdasasd"
        ];
    
        for word in incorrect_words {
            assert!(!wordbank.in_allowed_words(word));
        }
    }
}