use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Error as IoError, ErrorKind, Read};
use std::path::Path;

use rand::Rng;

fn main() {
    let path: &Path = Path::new(r"./data/words_alpha.txt");

    let word: String = draw_word(path);
    println!("word selected: {}", &word);
    
    let data: Vec<(usize, char)> = word.char_indices().collect();
    let mut display: Vec<char> = vec!['*'; data.len()];
    
    loop {
        guess(&word, user_input(), &mut display, &data); 
    }
}

fn user_input() -> char {
    println!("Input your guess - one letter only!");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
    input
        .trim()
        .chars()
        .next()
        .expect("not a valid input -- aA-zZ letters only")
}

fn guess(word: &String, user_guess: char, display: &mut Vec<char>, data: &Vec<(usize, char)>) {
    if word.contains(user_guess) {
        println!("Correct! You have correctly guessed a letter!");
        let (correct_letter, _): (Vec<(usize, char)>, _) 
            = data.iter().partition(|tup| { let (_, letter) = tup;
                                            *letter == user_guess 
                                          });
        for (idx, letter) in correct_letter.iter() {
            display[*idx] = *letter;
        }
        println!("The word is: {:#?}", display);
    } else {
        println!("Incorrect! This letter is not in the word.");
    }
}

fn draw_word(filepath: &Path) -> String {
    let mut rng = rand::thread_rng();

    let file = open_file(filepath);

    let words: Vec<String> = read_input(file).expect("could not create vec of strings");
    let word: &String = &words[rng.gen_range(0, &words.len()) as usize];
    word.to_owned()
}

fn open_file(path: &Path) -> File {
    let display = path.display();
    match File::open(&path) {
        Ok(file) => file,
        Err(stderr) => panic!("Could not open {}: {}", display, stderr.to_string()),
    }
}

fn read_input<R: Read>(file: R) -> Result<Vec<String>, IoError> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|row| {
            row.and_then(|data| {
            data.parse::<String>()
                .map_err(|err| IoError::new(ErrorKind::InvalidData, err))
                })
            })
        .collect()
}