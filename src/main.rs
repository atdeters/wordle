use std::collections::HashSet;
use colored::Colorize;
use console::Term;
use crossterm::{cursor, execute};

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum CharStatus {
    NotInWord = 0,
    WrongPos = 1,
    RightPos = 2
}
use rand::{Rng, rng};

fn print_gamestate(buffer: [[(char, CharStatus); 5]; 6]) -> ()
{
    print!("\x1B[2J\x1B[H");
    for line in buffer {
        for tup in line {
            match tup.1 {
                CharStatus::RightPos => {
                    print!("{}", tup.0
                                    .to_string()
                                    .bold()
                                    .black()
                                    .on_truecolor(128, 239, 128));
                }
                CharStatus::WrongPos => {
                    print!("{}", tup.0
                                    .to_string()
                                    .bold()
                                    .black()
                                    .on_truecolor(255, 206, 27));
                }
                _ => {
                    print!("{}", tup.0);
                }
            }
        }
        println!("");
    }
}

fn main() {
	// Get binary and store the contents from the text file into a string
	let	words: &'static str = include_str!("wordlists/words.txt");
	let tmp_dict: HashSet<&str> = words.lines().collect();
    let mut dict: HashSet<&str> = Default::default();
    for word in &tmp_dict {
        if word.len() == 5 {
            dict.insert(word);
        }
    }
    if dict.len() == 0 {
        eprintln!("Empty wordlist. Exiting game!");
        std::process::exit(1);
    }


    let mut buffer: [[(char, CharStatus); 5]; 6] = [[('_', CharStatus::NotInWord); 5]; 6];

    let index = rng().random_range(0..dict.len());
    let word_to_find: &str = dict
								.iter()
								.nth(index)
								.unwrap();

    // Create a counter for each char in the word_to_find
    let mut char_counter_wtf: [u8; 26] = [0; 26];
    for char in word_to_find.chars() {
        char_counter_wtf[char as usize - 'a' as usize] += 1;
    }

    let mut char_nb: usize;
    let mut term = Term::stdout(); // Terminal used to read input from user
    let _ = execute!(term, cursor::Hide);
    for i in 0..6 {
        print_gamestate(buffer);
        // Let user build the word in the buffer
        let mut idx: usize = 0;
        let mut key_in: console::Key;
        let mut char_in: char = '_';
        loop {
            while char_in != '\n' {
                key_in = term
                            .read_key()
                            .expect("Reason"); // TODO: Read what this is all about

                // Turn key into char
                match key_in {
                    console::Key::Char(c) => {
                        char_in = c;
                        char_in.make_ascii_lowercase();
                    }
                    console::Key::Backspace => {
                        char_in = 127 as char;
                    }
                    console::Key::Enter => {
                        char_in = '\n';
                    }
                    _ => {
                        char_in = '_';
                    }
                }

                if char_in == 127 as char && idx > 0 {
                    idx -= 1;
                    buffer[i][idx].0 = '_';
                }
                else if idx <= 4 && char_in.is_ascii_alphabetic() {
                    buffer[i][idx].0 = char_in;
                    idx += 1;
                }
                print_gamestate(buffer);
            }

            // Create a string out of our current buffer
            let mut tmp_word: String = String::from(""); // = buffer[i].iter().collect();

            for char_tup in buffer[i] {
                tmp_word.push_str(&char_tup.0.to_string());
            }
            let current_word: &str = tmp_word.as_str();

            if idx != 5 {
                println!("Not enough letters");
            }
            else if !dict.contains(current_word) {
                println!("Word not in wordlist: {current_word}");
            }
            else {
                break;
            }
            char_in = '_';
        }

        // Reveal information about latest word
        char_nb = 0;
        let mut char_counter_curr = char_counter_wtf;
        let mut correct_chars: u8 = 0;
        for char_tup in buffer[i].iter_mut() {
            // Character in right position
            if word_to_find.chars().nth(char_nb) == Some(char_tup.0) {
                char_counter_curr[char_tup.0 as usize - 'a' as usize] -= 1;
                char_tup.1 = CharStatus::RightPos;
                correct_chars += 1;
            }
            // Character in word but wrong position
            else if char_counter_curr[char_tup.0 as usize - 'a' as usize] != 0 {
                char_counter_curr[char_tup.0 as usize - 'a' as usize] -= 1;
                char_tup.1 = CharStatus::WrongPos;
            }
            char_nb += 1;
        }
        if correct_chars == 5 {
            print_gamestate(buffer);
            println!("Won game");
            let _ = execute!(term, cursor::Show);
            std::process::exit(0);
        }
        println!("");
    }
    print_gamestate(buffer);
    println!("You lose. The word was \"{}\"", word_to_find);
    let _ = execute!(term, cursor::Show);
}
