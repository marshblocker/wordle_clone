extern crate colored;

use colored::*;

use std::io;
use std::{thread, time};

use crate::constants::{WORD_LENGTH, MAX_GUESSES};
use crate::guess_processor::IfWinner;
use crate::err::AppError;

pub type LetterColorMapping = (char, FontColors);
pub type GuessColorMapping = [LetterColorMapping; WORD_LENGTH];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontColors {
    Green,
    Blue,
    Gray,
}

pub struct Display {
    allowed_letters: Vec<char>,
    invalid_letters: Vec<char>,
    user_guess: GuessColorMapping,
    user_guess_arr: [GuessColorMapping; MAX_GUESSES],
}

impl Display {
    pub fn init() -> Display {
        let allowed_letters: Vec<char> = ('A'..='Z').collect();
        let invalid_letters: Vec<char> = Vec::new();
        let user_guess = [(' ', FontColors::Gray); WORD_LENGTH];
        let user_guess_arr = [user_guess; MAX_GUESSES];

        Display { allowed_letters, user_guess, invalid_letters, user_guess_arr }
    }
    
    pub fn get_allowed_letters(&self) -> &Vec<char> {
        &self.allowed_letters
    }

    pub fn get_invalid_letters(&self) -> &Vec<char> {
        &self.invalid_letters
    }

    pub fn get_user_guess(&mut self, user_guess: GuessColorMapping) {
        self.user_guess = user_guess;
    }

    pub fn get_user_guess_arr(&self) -> [GuessColorMapping; MAX_GUESSES] {
        self.user_guess_arr
    }

    pub fn update_allowed_letters(&mut self) {
        self.allowed_letters.retain(|l| !self.invalid_letters.contains(l));
    }

    pub fn update_invalid_letters(&mut self, invalid: char) {
        self.invalid_letters.push(invalid);
    }
    
    pub fn update_user_guess_arr(&mut self, guess_count: usize) {
        self.user_guess_arr[guess_count] = self.user_guess;
    }

    pub fn print_allowed_letters(&self) {
        print!("{}", "Available Letters:".underline().bold());
        print!("  ");

        for letter in 'A'..='Z' {
            if self.allowed_letters.contains(&letter) {
                print!("{} ", letter)
            } else {
                print!("  ");
            }
        }
        println!("\n");
    }

    pub fn print_all_guesses(&self) {
        for guess in self.user_guess_arr {
            print!("\t\t\t");

            for letter in guess {
                print!("{} ", Self::colorize(letter));
            }
            println!("\n");
        }
    }

    fn colorize(letter: LetterColorMapping) -> ColoredString {
        let (chr, color) = letter;

        match color {
            FontColors::Green => chr.to_string().green().bold().underline(),
            FontColors::Blue => chr.to_string().blue().underline(),
            FontColors::Gray => chr.to_string().normal().bold().underline(),
        }
    }
}

pub fn display_start_screen() -> char {
    println!("\nLet's play Wordle!\n");
    println!(
        "Press {} to play the game or press {} to display the mechanics of the game.\n", 
        "P".underline(), "H".underline() 
    );
    
    loop {
        let mut command = String::new();

        io::stdin()
          .read_line(&mut command)
          .expect("Error reading input.");

        match is_command_valid(&command, vec!['P', 'H']) {
            Ok(cmd) => return cmd,
            Err(err) => eprintln!("{}\n", err),
        }
    }
}

pub fn display_help() {
    clear_screen();

    println!("
    Game Mechanics: Guess the five-letter word in five tries. \
    Your guess will change color depending on its correctness.\n\n\
    For example, if the unknown word is 'altar', and your guess is \n\
    'later', then your guess will be displayed as {} {} {} {} {}, \n\
    where 'L' and 'A' are colored {} since they can be found in the \n\
    word 'altar' {} they are in the wrong position ('L' and 'A' must \n\
    swap to be in correct position), 'T' and 'R' are colored {} since \n\
    they can be found in the word 'altar' {} they are in the \n\
    correct position, while 'E' is colored {} since it cannot be found \n\
    in the word 'altar'.\n\nUse these color hints to guess the unknown word!\n", 
    "L".blue().bold(), "A".blue().bold(), "T".green().bold(), "E".bold(), 
    "R".green().bold(), "BLUE".blue().bold(), "but".italic(), 
    "GREEN".green().bold(), "and".italic(), "GRAY".normal().bold()
    );

    
    loop {
        println!("Press P to play the game:");
        
        let mut command = String::new();

        io::stdin()
          .read_line(&mut command)
          .expect("Error reading input.");

        match is_command_valid(&command, vec!['P']) {
            Ok(_) => return,
            Err(err) => eprintln!("{}\n", err),
        }
    }
}

pub fn display_guesses_left(guesses_left: u8) {
    if guesses_left == 0 {
        println!(
            "Number of guesses left: {}. {}", 
            "0".red().bold(), "GAME OVER".red().bold()
        );
    } else { println!("Number of guesses left: {}", guesses_left); }
}

// Don't mind the maths, it just prints a fancy animation of the ending screen.
pub fn display_end_screen(winner: IfWinner, unknown_answer: &str) {
    const WSPACE: u16 = 100;

    let sleep_sec = time::Duration::from_secs(3);
    thread::sleep(sleep_sec);

    clear_screen();

    let mut n = match winner {
        true   => 0,
        false => WSPACE,
    };

    loop {
        if winner && n == WSPACE {
            n = 0;
        } else if !winner && n == 0 {
            n = WSPACE;
        }
        
        let sleep_milli = time::Duration::from_millis(75);
        thread::sleep(sleep_milli);

        for _ in 0..n {
            print!(" ");
        }

        if winner {
            print!("{}", "You won the game!\n".green().bold());
        } else {
            print!("{}", format!(
                "You lost! The correct answer is {}.\n", unknown_answer).red().bold()
            );
        }

        if winner { n += 1; } else { n -= 1; }
    }
}

pub fn clear_screen() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}


fn is_command_valid(command: &str, valid_commands: Vec<char>) -> Result<char, String> {
    let command: char = match command.trim().parse() {
        Ok(cmd) => cmd,
        Err(_)  => return Err(AppError::InvalidCommandErr.to_str(Some(&valid_commands))),
    };

    let command = command.to_ascii_uppercase();

    if !valid_commands.contains(&command) {
        return Err(AppError::InvalidCommandErr.to_str(Some(&valid_commands)));
    }
    
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_command_valid() {
        let command: [&str; 4] = ["p", "h", "P", "H"];

        for c in command {
            let c_as_char = c.chars().next().unwrap().to_ascii_uppercase();

            assert_eq!(
                Ok(c_as_char), 
                is_command_valid(&c.to_string(), vec!['P', 'H'])
            );
        }
        
        let command: [&str; 5] = ["asd", "a123d,", "@!#$", " ", "."];
        
        for c in command {
            assert_eq!(
                Err(AppError::InvalidCommandErr.to_str(Some(&vec!['P', 'H']))), 
                is_command_valid(&c.to_string(), vec!['P', 'H'])
            );
        }
    }
}