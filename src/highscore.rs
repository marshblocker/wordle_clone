use std::io::prelude::*;
use std::io::ErrorKind;
use std::fs::{self, File};
use std::env;

#[derive(Debug)]
pub struct UserScore {
    username: String,
    score: u8
}

impl UserScore {
    pub fn new(username: String, score: u8) -> UserScore {
        UserScore { username, score }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_score(&self) -> u8 {
        self.score
    }
}

#[derive(Debug)]
pub struct HighScores {
    high_scores: Vec<UserScore>,
    highscore_text_path: std::path::PathBuf
}

impl HighScores {
    pub fn init() -> HighScores {
        let mut curr_exe_dir = env::current_exe()
        .expect("Failed to get the path of the program's executable.");
        
        curr_exe_dir.pop();
        curr_exe_dir.push("wc_resources");

        let game_resources_dir_path = curr_exe_dir.clone();

        if !game_resources_dir_path.is_dir() {
            fs::create_dir(game_resources_dir_path)
                .expect("Failed to create a directory for the game's resources.");
        }

        let mut highscore_text_path = curr_exe_dir.clone();

        highscore_text_path.push("highscore.txt");

        let high_scores_str = if highscore_text_path.is_file() {
            let high_scores_str = fs::read_to_string(highscore_text_path.clone())
                .expect("Failed to read highscore.txt");
            
            if high_scores_str.is_empty() {
                return HighScores { high_scores: Vec::new(), highscore_text_path };
            }

            high_scores_str
        } else {
            fs::File::create(highscore_text_path.clone())
                .expect("Failed to create new file highscore.txt");
            
            return HighScores { high_scores: Vec::new(), highscore_text_path };
        };
        
        
        let mut high_scores_clean: Vec<UserScore> = Vec::new();
        let high_scores_raw: Vec<Vec<&str>> = high_scores_str
            .lines()
            .map(|line| line.split_ascii_whitespace().collect())
            .collect();


        for high_score in high_scores_raw {
            if high_score.len() != 2 {
                panic!("The high score {:?} does not have two elements only.", high_score);
            }

            let name = high_score[0].to_string();
            let score: u8 = high_score[1]
                .trim()
                .parse()
                .expect("Cannot parse one of the scores in highscore.txt");

            high_scores_clean.push(UserScore::new(name, score));
        }

        HighScores { high_scores: high_scores_clean, highscore_text_path }
    }

    pub fn get_high_scores(&self) -> &Vec<UserScore> {
        &self.high_scores
    }

    pub fn try_insert_new_score(&mut self, new_score: UserScore) {
        let mut pos: Option<usize> = None;

        if self.high_scores.is_empty() {
            self.high_scores.push(new_score);
            self.update_high_scores();
            return;
        }

        for (i, userscore) in self.high_scores.iter().enumerate() {
            if new_score.score > userscore.score { 
                pos = Some(i); 
                break;
            }
        }
        
        if let Some(i) = pos { 
            self.high_scores.insert(i, new_score); 
        } else { 
            self.high_scores.push(new_score); 
        }

        if self.high_scores.len() > 5 {
            self.high_scores.pop();
        }

        self.update_high_scores();
    }

    fn update_high_scores(&self) {
        let mut highscore_text = File::create(self.highscore_text_path.clone())
            .expect("'highscore.txt' is not a valid file path");

        let mut buf = String::new();

        for userscore in &self.high_scores {
            buf.push_str(format!(
                "{} {}\n", 
                userscore.get_username(), userscore.get_score()
            ).as_str());
        }

        // Removes the last newline char.
        buf.pop();
        
        let buf = buf.as_bytes();

        while let Err(err) = highscore_text.write(buf) {
            match err.kind() {
                ErrorKind::Interrupted => continue,
                _ => eprintln!("IO Error: {}", err),
            }
        }
    }
}