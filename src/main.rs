use std::collections::HashSet;
use colored::Colorize;
use console::Term;

fn main() {
	// Get binary and store the contents from the text file into a string
	let	words: &'static str = include_str!("wordlists/words.txt");

	// Split into words and collect them into a HashSet.
	let dict: HashSet<&str> = words.lines().collect();

    let mut buffer: [[char; 5]; 6] = [['_'; 5]; 6];

    // TODO: Get word of the day aka replace "harsh" with random word of dict
    let word_to_find: &str = "harsh";

    // Create a counter for each char in the word_to_find
    let mut char_counter_wtf: [u8; 26] = [0; 26];
    for char in word_to_find.chars() {
        char_counter_wtf[char as usize - 'a' as usize] += 1;
    }

    let mut char_nb: usize;
    let term = Term::stdout(); // Terminal used to read input from user
    for i in 0..6 {

        // Let user build the word in the buffer
        let mut idx: usize = 0;
        let mut key_in: console::Key;
        let mut char_in: char = '_';
        loop {
            while char_in != '\n' {
                // TODO: Turn everything into uppercase
                key_in = term
                            .read_key()
                            .expect("Reason"); // TODO: Read what this is all about
               
                // Turn key into char
                match key_in {
                    console::Key::Char(c) => {
                        char_in = c;
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
                    buffer[i][idx] = '_';
                }
                // TODO: Make sure only english alphabet is allowed!
                else if idx <= 4 && char_in.is_alphabetic() {
                    buffer[i][idx] = char_in;
                    idx += 1;
                }
                print!("\x1B[2J\x1B[H");
                for line in buffer {
                    println!("{:?}", line);
                }
            }

            // Create a string out of our current buffer
            let tmp_word: String = buffer[i].iter().collect();
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
        for character in buffer[i] {
            // Character in right position
            if word_to_find.chars().nth(char_nb) == Some(character) {
                print!("{}", character
                                .to_string()
                                .bold()
                                .black()
                                .on_truecolor(128, 239, 128));
                char_counter_curr[character as usize - 'a' as usize] -= 1;
                correct_chars += 1;
            }
            // Character in word but wrong position
            else if char_counter_curr[character as usize - 'a' as usize] != 0 {
                print!("{}", character
                                .to_string()
                                .bold()
                                .black()
                                .on_truecolor(255, 206, 27));
                char_counter_curr[character as usize - 'a' as usize] -= 1;
            }
            else {
                print!("{character}");
            }
            print!(" ");
            char_nb += 1;
        }
        if correct_chars == 5 {
            println!("Won game");
            std::process::exit(0);
        }
        println!("");
    }
    println!("You lose. The word was \"{}\"", word_to_find);
}
