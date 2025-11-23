use std::collections::HashSet;
use macroquad::prelude::*;
use macroquad::rand::rand;
use macroquad::rand::srand;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum CharStatus {
    NotInWord = 0,
    WrongPos = 1,
    RightPos = 2,
    NotRevealed = 3
}

fn print_gamestate_win(t_buffer: [[(char, CharStatus); 5]; 6], t_text: &str) -> ()
{
    // === COLORS ===
    const COL_BACK: Color = Color::new(18.0 / 255.0, 18.0 / 255.0, 19.0 / 255.0, 1.00);
    const COL_RIGHT_POS: Color = Color::new(83.0 / 255.0, 141.0 / 255.0, 78.0 / 255.0, 1.0);
    const COL_WRONG_POS: Color = Color::new(181.0 / 255.0, 160.0 / 255.0, 59.0 / 255.0, 1.0);
    // const COL_UNUSED = Color::new(129.0 / 255.0, 131.0 / 255.0, 132.0 / 255.0, 1.0);
    const COL_GRID: Color = Color::new(58.0 / 255.0, 58.0 / 255.0, 60.0 / 255.0, 1.00);
    const COL_NOTINWORD: Color = COL_GRID;

    // === SIZES ===
    const BLOCK_SIZE: f32 = 60.0;
    const GRID_OFFSET_Y: f32 = 50.0;
    const GRID_THICC: f32 = 4.0;
    const GRID_GAP: f32 = 5.0;
    const FONT_SIZE: u16 = 80;
    const INFO_FONT_SIZE: u16 = 30;
    const INFO_TEXT_GAP: f32 = 50.0;

    clear_background(COL_BACK);
    for i in 0..6 {
        for j in -2isize..3 {

            // Char and its variables to print it
            let mut curr_char: String = t_buffer[i][(j + 2) as usize].0.to_string();
            curr_char.make_ascii_uppercase();
            let center: Vec2 = get_text_center(&curr_char, Option::None, FONT_SIZE, 1.0, 0.0);
            let text_x: f32 = screen_width() / 2.0 + j as f32 * (BLOCK_SIZE + GRID_GAP) - center.x;
            let text_y: f32 = GRID_OFFSET_Y + i as f32 * (BLOCK_SIZE + GRID_GAP) + BLOCK_SIZE / 2.0 - center.y;

            let status: CharStatus = t_buffer[i][(j+2) as usize].1;

            // Variables for the boxes
            let box_x: f32 = screen_width() / 2.0 - BLOCK_SIZE / 2.0 + j as f32 * (BLOCK_SIZE + GRID_GAP);
            let box_y: f32 = GRID_OFFSET_Y + i as f32 * (BLOCK_SIZE + GRID_GAP);

            match status {
                CharStatus::NotRevealed => {
                    draw_rectangle_lines(box_x, box_y, BLOCK_SIZE, BLOCK_SIZE, GRID_THICC, COL_GRID);
                    if curr_char != "_" {
                        draw_text(&curr_char, text_x, text_y, FONT_SIZE.into(),WHITE);
                    }
                }
                CharStatus::WrongPos => {
                    draw_rectangle(box_x, box_y, BLOCK_SIZE, BLOCK_SIZE, COL_WRONG_POS);
                    draw_text(&curr_char, text_x, text_y, FONT_SIZE.into(), WHITE);
                }
                CharStatus::RightPos => {
                    draw_rectangle(box_x, box_y, BLOCK_SIZE, BLOCK_SIZE, COL_RIGHT_POS);
                    draw_text(&curr_char, text_x, text_y, FONT_SIZE.into(), WHITE);
                }
                CharStatus::NotInWord => {
                    draw_rectangle(box_x, box_y, BLOCK_SIZE, BLOCK_SIZE, COL_NOTINWORD);
                    draw_text(&curr_char, text_x, text_y, FONT_SIZE.into(),  WHITE);
                }
            }
        }
    }

    // Display info_text
    let center = get_text_center(t_text, Option::None, INFO_FONT_SIZE, 1.0, 0.0);
    draw_text(t_text,
        screen_width() / 2.0 - center.x,
        (GRID_OFFSET_Y + 6.0 * (BLOCK_SIZE + GRID_GAP)) + center.y + INFO_TEXT_GAP,
                INFO_FONT_SIZE.into(),
                WHITE);
}

const CHEATS_ON: bool = true;

// TODO: Try to add a proper font

#[macroquad::main("ft_wordle")]
async fn main() {

    let mut game_over: bool = false;

    // Create dict from file + validations
	let words: &'static str = include_str!("wordlists/words.txt");
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

    // Chose random word of the day
    srand(macroquad::miniquad::date::now() as u64);
    let index = rand() % dict.len() as u32;
    let word_to_find: &str = dict
                                .iter()
                                .nth(index as usize)
                                .unwrap();

    if CHEATS_ON == true {
        println!("{word_to_find}");
    }

    // Create a counter for each char in the word_to_find
    let mut char_counter_wtf: [u8; 26] = [0; 26];
    for char in word_to_find.chars() {
        char_counter_wtf[char as usize - 'a' as usize] += 1;
    }

    let mut buffer: [[(char, CharStatus); 5]; 6] = [[('_', CharStatus::NotRevealed); 5]; 6];
    let mut buff_idx_y: usize = 0;
    let mut buff_idx_x: usize = 0;
    let mut info_text: String = String::from("");

    // Main game loop
    loop {
        let curr = get_char_pressed();
        match curr {
            Some(mut c) => {
                if c.is_ascii_alphabetic() && buff_idx_x < 5 {
                    println!("Log: {c} pressed");
                    if !game_over {
                    c.make_ascii_lowercase();
                    buffer[buff_idx_y][buff_idx_x].0 = c;
                    buff_idx_x += 1;
                    info_text = "".to_string();
                    }
                }
            }
            None => {}
        }

        if is_key_pressed(KeyCode::Backspace) && buff_idx_x > 0 {
            println!("Log: Backspace pressed");
            if !game_over {
                info_text = "".to_string();
                buff_idx_x -= 1;
                buffer[buff_idx_y][buff_idx_x].0 = '_';
            }
        }

        if is_key_pressed(KeyCode::Enter) {
            println!("Log: Enter pressed");
            // Not big enough
            if buff_idx_x < 5 && !game_over {
                info_text = "Not enough letters".to_string();
                eprintln!("Not enough letters");
            }
            else if !game_over {
                // Create a string out of our current buffer
                let mut tmp_word: String = String::from("");
                for char_tup in buffer[buff_idx_y] {
                    tmp_word.push_str(&char_tup.0.to_string());
                }
                let current_word: &str = tmp_word.as_str();

                if !dict.contains(current_word) {
                    info_text = format!("Wort not in wordlist: {current_word}").to_string();
                    eprintln!("Word not in wordlist: {current_word}");
                }
                else if !game_over {
                    // Reveal information about latest word
                    let mut char_counter_curr = char_counter_wtf;
                    let mut correct_chars: u8 = 0;

                    /*
                    * We do this in 2 different loops so that if correct characters
                    * in the wrong position followed by ones in the right one do not
                    * become colored incorrectly
                    */
                    let mut char_nb: usize = 0;
                    let mut counter_idx: usize;
                    // Character in right position
                    for char_tup in buffer[buff_idx_y].iter_mut() {
                        if word_to_find.chars().nth(char_nb) == Some(char_tup.0) {
                            counter_idx = char_tup.0 as usize - 'a' as usize;
                            if char_counter_curr[counter_idx] > 0 {
                                char_counter_curr[counter_idx] -= 1;
                                char_tup.1 = CharStatus::RightPos;
                                correct_chars += 1;
                            }
                        }
                        char_nb += 1;
                    }

                    // Character in word but wrong position
                    for char_tup in buffer[buff_idx_y].iter_mut() {
                        counter_idx = char_tup.0 as usize - 'a' as usize;
                        if char_counter_curr[counter_idx] > 0
                            && char_tup.1 != CharStatus::RightPos {
                            char_counter_curr[counter_idx] -= 1;
                            char_tup.1 = CharStatus::WrongPos;
                        }
                        else if char_tup.1 != CharStatus::RightPos {
                            char_tup.1 = CharStatus::NotInWord;
                        }
                    }

                    if correct_chars == 5 {
                        game_over = true;
                        if buff_idx_y == 0 {
                            info_text = "Wow! You won on the first try!".to_string();
                        }
                        else {
                            info_text = "Game won. Congratulations!".to_string();
                        }
                        println!("Won game");
                    }

                    buff_idx_y += 1;
                    buff_idx_x = 0;
                    if buff_idx_y == 6 {
                        game_over = true;
                        info_text = format!("You lose! The word was {word_to_find}").to_string();
                        println!("You lose. The word was {}", word_to_find);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            println!("Log: Escape pressed");
            info_text = "There is no escape!".to_string();
        }

        for touch in touches() {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (WHITE, 60.0),
                TouchPhase::Moved => (YELLOW, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),
                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
        }

        print_gamestate_win(buffer, &info_text);
        next_frame().await;
    }
}
