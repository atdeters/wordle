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

    buffer[1][2] = 'a';

    for i in 0..5 {

        // Check current word
        for j in 0..4 {
            // TODO: 1. Check if the character matches the word_to_find
                // if yes: print green
                // if no:
                    // if does exist: print yellow
                    // else: print gray
            if word_to_find.chars().nth(j) == Some(buffer[i][j]) {
                print!("\x1b[1;32m");
            }
            print!("{}", buffer[i][j]);
            print!("\x1b[0m");
            print!(" ");
        }
        println!("");
    }
}
