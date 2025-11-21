// use std::collections::HashSet;
use std::collections::HashSet;

fn main() {
	// Get binary and store the contents from the text file into a string
	let	words: &'static str = include_str!("wordlists/words.txt");

	// Split into words and collect them into a HashSet.
	let dict: HashSet<&str> = words.lines().collect();
	
    let mut buffer: [[char; 5]; 6] = [['_'; 5]; 6];
	println!("{:?}", dict);
    // TODO: Get word of the day
    // let mut _wordlist: HashSet<String> = HashSet::new();

    let word_to_find: &'static str = "harsh";

    buffer[1][1] = 'a';
    buffer[1][2] = 'o';

    let mut char_nb: usize;
    for elem in buffer {
        // Check if elem is part of the wordlist
        let current_word: String = elem.iter().collect();
        char_nb = 0;
        for character in elem {
            // Character in right position
            if word_to_find.chars().nth(char_nb) == Some(character) {
                print!("\x1b[1;38;5;0;48;2;128;239;128m");
            }
            // Character in word but wrong position
            else if character == 'o' {
                print!("\x1b[1;38;5;0;48;2;255;206;27m");
            }
            print!("{}", character);
            print!("\x1b[0m");
            print!(" ");
            char_nb += 1;
        }
        println!("");
    }
}
