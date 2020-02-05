use std::io::{stdin, Error as IoError, ErrorKind};

use rand::Rng;

fn main() {
    loop {
        println!("Welcome to Terminal Hangman!");
        println!("... Loading...");

        let all_words = include_str!("words_alpha.txt");
        let _wd: String = draw_word(all_words);
        let _dt: Vec<(usize, char)> = _wd.char_indices().collect();
        let _di: Vec<char> = vec!['_'; _dt.len()];

        let mut gd: GameData = GameData {
            word: _wd,
            data: _dt,
            display: _di,
            attempts: vec![],
            try_count: 0,
            max_count: 5,
        };

        println!("... Entering game.\n");
        loop {
            gd.display_guesses();
            match guess(&mut gd, user_input()) {
                GuessStatus::Correct => {
                    // If it's true that the output still contains blanks, then the continue the game.
                    match gd.has_blanks() {
                        GameState::InProgress => continue,
                        GameState::Complete => break,
                    }
                }
                GuessStatus::Incorrect => match gd.check_count() {
                    GameState::InProgress => continue,
                    GameState::Complete => break,
                },
            }
        }

        println!("Thanks for playing! Input [Any Key] to try again, or [n] to exit program.");
        match user_input() {
            'n' => {
                println!("Ok! Exiting program.\n");
                break;
            }
            _ => {
                println!("Ok! Beginning again...\n");
                continue;
            }
        }
    }
}

struct GameData {
    word: String,
    data: Vec<(usize, char)>,
    display: Vec<char>,
    attempts: Vec<char>,
    try_count: i8,
    max_count: i8,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
enum GuessStatus {
    Correct,
    Incorrect,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
enum GameState {
    InProgress,
    Complete,
}

impl GameData {
    fn display_guesses(&self) {
        println!(
            "Input your guess - one letter only!\nLetters guessed so far: [{}]",
            self.attempts.iter().collect::<String>()
        );
    }

    fn has_blanks(&self) -> GameState {
        if self.display.contains(&'_') {
            GameState::InProgress
        } else {
            println!("Congrats! You've successfully guessed the word!\n");
            GameState::Complete
        }
    }

    fn check_count(&mut self) -> GameState {
        self.try_count += 1;
        println!("Guesses remaining: {}\n", self.max_count - self.try_count);
        if self.try_count >= self.max_count {
            println!(
                "You're out of guesses! Game over!\nThe word was... '{}'\n",
                self.word
            );
            GameState::Complete
        } else {
            GameState::InProgress
        }
    }
}

// Takes the first char of any input only
fn user_input() -> char {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
    input
        .trim()
        .chars()
        .next()
        .expect("not a valid input -- aA-zZ letters only")
}

fn guess(gd: &mut GameData, user_guess: char) -> GuessStatus {
    let word: &str = &gd.word;
    let display: &mut Vec<char> = &mut gd.display;
    let attempts: &mut Vec<char> = &mut gd.attempts;
    let data: &[(usize, char)] = &mut gd.data;

    if word.contains(user_guess) {
        println!("Correct! You have correctly guessed a letter!");
        let (correct_letter, _): (Vec<(usize, char)>, _) = data.iter().partition(|tup| {
            let (_, letter) = tup;
            *letter == user_guess
        });
        for (idx, letter) in correct_letter.iter() {
            display[*idx] = *letter;
        }
        println!("The word is: {:?}\n", display.iter().collect::<String>());
        GuessStatus::Correct
    } else {
        println!("Incorrect! This letter is not in the word.");
        attempts.push(user_guess);
        GuessStatus::Incorrect
    }
}

fn draw_word(input: &str) -> String {
    let mut rng = rand::thread_rng();
    let words: Vec<String> = read_input(input).expect("could not create vec of strings");
    words[rng.gen_range(0, &words.len()) as usize].to_owned()
}

fn read_input(file: &str) -> Result<Vec<String>, IoError> {
    file.lines()
        .map(|row| {
            row.parse::<String>()
                .map_err(|err| IoError::new(ErrorKind::InvalidData, err))
        })
        .collect()
}
