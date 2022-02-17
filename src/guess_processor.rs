use std::collections::HashMap;

use crate::display::{GuessColorMapping, FontColors, Display};
use crate::constants::WORD_LENGTH;

pub type IfWinner = bool;

// Maps each letter of the user's guess to the three colors that represents
// how correct they are with respect to the letters of the unknown word. 
// This also checks if the user correctly guesses the unknown word.
pub fn process_guess(
        guess: &str, 
        unknown_word: &str, 
        display: &mut Display,
) -> (GuessColorMapping, IfWinner) {
    assert_eq!(guess.to_lowercase().as_str(), guess);
    assert_eq!(unknown_word.to_lowercase().as_str(), unknown_word);

    let mut gcm: GuessColorMapping = [(' ', FontColors::Gray); WORD_LENGTH];
    let mut ifwinner = false;

    let mut letter_occurrences: HashMap<char, u8> = HashMap::new();

    for letter in unknown_word.chars() {
        *letter_occurrences.entry(letter).or_insert(0) += 1;
    }

    let guess_arr: Vec<char> = guess.chars().collect();
    let unknown_word_arr: Vec<char> = unknown_word.chars().collect();

    assert_eq!(guess_arr.len(), unknown_word_arr.len());

    color_correct_guess_letters(
        &guess_arr, &unknown_word_arr, 
        &mut letter_occurrences, &mut gcm, 
        &mut ifwinner
    );

    color_other_guess_letters(
        &guess_arr, &unknown_word_arr, 
        &mut letter_occurrences, display,
        &mut gcm
    );

    for letter_color_mapping in gcm {
        assert_ne!((' ', FontColors::Gray), letter_color_mapping);
    }

    (gcm, ifwinner)
}

// If the i-th letter in the user's guess matches the i-th letter of the unknown
// word, this maps the letter to color GREEN.
fn color_correct_guess_letters(
        guess_arr: &[char],
        unknown_word_arr: &[char],
        letter_occurrences: &mut HashMap<char, u8>,
        gcm: &mut GuessColorMapping,
        ifwinner: &mut IfWinner
) {
    let mut correct_letters = 0;

    for i in 0..guess_arr.len() {
        if guess_arr[i] == unknown_word_arr[i] {
            let correct: char = guess_arr[i];

            *letter_occurrences.entry(correct).or_insert(0) -= 1;
            gcm[i] = (correct.to_ascii_uppercase(), FontColors::Green);
            correct_letters += 1;
        }
    }
    if correct_letters == WORD_LENGTH {
        *ifwinner = true;
    }
}

// If the i-th letter in the user's guess matches with a j-th letter in the unknown
// word, where i != j, then this maps the letter to color BLUE. But if the i-th
// letter in the user's guess does not match ANY letter of the unknown word, then
// this just maps it to gray (or basically not changing its color).
fn color_other_guess_letters(
        guess_arr: &[char],
        unknown_word_arr: &[char],
        letter_occurrences: &mut HashMap<char, u8>,
        display: &mut Display,
        gcm: &mut GuessColorMapping,
) {
    for i in 0..guess_arr.len() {
        let incorrect: char = guess_arr[i];
        
        if incorrect != unknown_word_arr[i] {
            if unknown_word_arr.contains(&incorrect) && letter_occurrences[&incorrect] > 0 {
                *letter_occurrences.entry(incorrect)
                                   .or_insert(0) -= 1;
                gcm[i] = (incorrect.to_ascii_uppercase(), FontColors::Blue);
            } else {
                gcm[i] = (incorrect.to_ascii_uppercase(), FontColors::Gray);     
                
                if !display.get_invalid_letters().contains(&incorrect) {
                    display.update_invalid_letters(incorrect.to_ascii_uppercase());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_guess1() {
        let mut display = Display::init();

        let guess: [&str; 5] = [
            "hello",
            "catch",
            "swear",
            "slick",
            "zebra"
        ];
        
        let unknown_word: [&str; 5] = [
            "hello",
            "catch",
            "swear",
            "slick",
            "zebra"
        ];

        for i in 0..guess.len() {
            let guess = guess[i].to_string();
            let unknown_word = unknown_word[i].to_string();

            let (gcm, ifwinner) = process_guess(&guess, &unknown_word, &mut display);
            let color_vec = get_color_vec(&gcm);
            
            assert_eq!(
                (vec![FontColors::Green; WORD_LENGTH], true), 
                (color_vec, ifwinner)
            );
        }
    }

    #[test]
    fn test_process_guess2() {
        let mut display = Display::init();

        let guess = "tenet".to_string();
        let unknown_word = "catch".to_string();
        
        let (gcm, ifwinner) = process_guess(&guess, &unknown_word, &mut display);
        let color_vec = get_color_vec(&gcm);

        assert_eq!(
            (vec![
                FontColors::Blue,
                FontColors::Gray,
                FontColors::Gray,
                FontColors::Gray,
                FontColors::Gray,
            ], false),
            (color_vec, ifwinner)
        );
    }

    #[test]
    fn test_process_guess3() {
        let mut display = Display::init();

        let guess = "shell".to_string();
        let unknown_word = "hello".to_string();

        let (gcm, ifwinner) = process_guess(&guess, &unknown_word, &mut display);
        let color_vec = get_color_vec(&gcm);

        assert_eq!(
            (vec![
                FontColors::Gray,
                FontColors::Blue,
                FontColors::Blue,
                FontColors::Green,
                FontColors::Blue,
            ], false),
            (color_vec, ifwinner)
        );
    }

    fn get_color_vec(gcm: &GuessColorMapping) -> Vec<FontColors> {
        let color_vec: Vec<FontColors> = gcm.iter()
                                            .map(|tup| tup.1)
                                            .collect();

        color_vec
    }
}