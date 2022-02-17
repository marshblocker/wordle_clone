use std::io;

use crate::constants::WORD_LENGTH;
use crate::err::AppError;

use crate::wordbank::WordBank;

// Returns a lower-cased version of the validated user input.
pub fn get_user_guess(wordbank: &WordBank) -> Result<String, io::Error> {
    let guess: String = loop {
        println!("Your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)?;

        if let Err(err) = is_guess_valid(&mut guess, wordbank) {
            eprintln!("\n{}", err);
            continue;
        }

        println!("{}", guess);
        break guess;
    };

    Ok(guess)
}

fn is_guess_valid(
        guess: &mut String, 
        wordbank: &WordBank
        ) -> Result<(), String> {  
    *guess = guess.trim().to_string();

    if guess.len() != WORD_LENGTH {
        return Err(AppError::InvalidGuessLengthErr.to_str::<()>(None));
    }

    if !guess.is_ascii() || !each_char_is_alpha(guess) {
        return Err(AppError::NonAlphaGuessErr.to_str::<()>(None));
    }

    *guess = guess.to_lowercase();

    if !wordbank.in_allowed_words(guess) {
        return Err(AppError::NotEnglishGuessErr.to_str::<()>(None));
    }

    Ok(())
}

fn each_char_is_alpha(string: &str) -> bool {
    string.chars()
          .all(|c| matches!(c, 'a'..='z' | 'A'..='Z'))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_guess_valid() {
        let wordbank = WordBank::init().unwrap();
        let guess: [&str; 5] = [
            "longest",
            "absc5",
            "!@#$%",
            "zebrt",
            "zebra",
        ];

        for (i, word) in guess.iter().enumerate() {
            assert_eq!(
                match i {
                    0   => Err(AppError::InvalidGuessLengthErr.to_str::<()>(None)),
                    1|2 => Err(AppError::NonAlphaGuessErr.to_str::<()>(None)),
                    3   => Err(AppError::NotEnglishGuessErr.to_str::<()>(None)),
                    4   => Ok(()),
                    _   => panic!("Should not reach here!"),
                },
                is_guess_valid(&mut word.to_string(), &wordbank)
            );
        }
    }

    #[test]
    fn test_each_char_is_alpha() {
        let valid_strings: [&str; 5] = [
            "hello",
            "HELLO",
            "HeLlo",
            "QWERTYPOIQWE",
            "ZXCVCSsefcxbDSF"
        ];

        for string in valid_strings {
            assert!(each_char_is_alpha(string));
        }
        
        let invalid_strings: [&str; 5] = [
            "123456",
            "juo768!",
            "asd sdgf ewr",
            "    ",
            "!@#$%^&*("
        ];

        for string in invalid_strings {
            assert!(!each_char_is_alpha(string));
        }
    }
}