// use std::collections::HashSet;

fn main() {
    // Create a buffer to store our inputs
    let mut buffer: [[char; 5]; 6] = [['_'; 5]; 6];

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
