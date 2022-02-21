// Wordle made in Rust. (wow very original!)

extern crate colored;

use std::process;

use wordle_clone::wordbank::{WordBank};
use wordle_clone::user_input::{self, IfWinner};
use wordle_clone::display::{self, Display, GuessColorMapping};
use wordle_clone::constants::MAX_GUESSES;
use wordle_clone::highscore::{HighScores, UserScore};
use wordle_clone::utils;

fn main() {
    utils::clear_screen();

    let mut display: Display = Display::init();
    let wordbank = WordBank::init();
    let mut highscore = HighScores::init();
                           
    let mut winner: IfWinner = false;

    let mut guesses_left = MAX_GUESSES;
    let unknown_word: String = wordbank.get_random_word_in_unknown_words();

    let cmd: char = display::display_start_screen(highscore.get_high_scores());
    match cmd {
        'H' => display::display_help(),
        'Q' => process::exit(0),
        _   => (),
    }

    let username = user_input::get_username();
    
    utils::clear_screen();
    display.print_allowed_letters();
    display.print_all_guesses();
    display::display_guesses_left(guesses_left as u8);

    while guesses_left > 0 {
        let guess: String = user_input::get_user_guess(&wordbank)
                                                    .unwrap_or_else(|err| {
                                                        eprintln!("{}", err);
                                                        process::exit(1);
                                                    });
        let (gcm, winner_temp): (GuessColorMapping, IfWinner) = user_input::process_guess(
            &guess, &unknown_word, &mut display);

        winner = winner_temp;

        utils::clear_screen();
        
        display.update_allowed_letters();
        display.get_user_guess(gcm);
        display.update_user_guess_arr(MAX_GUESSES - guesses_left);
        guesses_left -= 1;
        display.print_allowed_letters();
        display.print_all_guesses();

        if winner { 
            let score = (guesses_left as u8) + 1;
            highscore.try_insert_new_score(UserScore::new(username.clone(), score));
            display::display_end_screen(winner, &unknown_word); 
        } 
        
        display::display_guesses_left(guesses_left as u8);

    }
    
    display::display_end_screen(winner, &unknown_word);
}
