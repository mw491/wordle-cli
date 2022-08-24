extern crate ncurses;
mod pick_word;

use ncurses::*;
use std::process;

const NORMAL: i16 = 1;
const GREEN: i16 = 2;
const YELLOW: i16 = 3;
const RED: i16 = 4;

struct Game {
    target_word: String,
    max_chances: usize,
    chances_used: usize,
    letters_typed: usize,
    max_letters: usize,
    words_typed: Vec<String>,
    running: bool,
}

fn quit(game: &mut Game) {
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    addstr("\nPress any key to quit, Press `r` to play again.");
    let key = getch() as u8 as char;
    if key == 'r' {
        *game = init_game();
        game.start();
    } else {
        endwin();
        process::exit(0);
    }
}

impl Game {
    fn print_words(&mut self) {
        for word in self.words_typed.iter() {
            let mut letters: Vec<String> = vec![];
            for letter in word.chars() {
                letters.push(String::from(letter));
            }

            for (i, letter) in letters.iter().enumerate() {
                let color = if letter == &String::from(self.target_word.chars().nth(i).unwrap()) {
                    GREEN
                } else if self.target_word.contains(letter) {
                    YELLOW
                } else {
                    NORMAL
                };
                attron(COLOR_PAIR(color));
                mv(
                    (self.chances_used).try_into().unwrap(),
                    i.try_into().unwrap(),
                );
                addstr(letter);
                attroff(COLOR_PAIR(color));
            }
        }
        if self.words_typed.len() > 0 {
            let word = self.words_typed[self.words_typed.len() - 1].clone();
            let plural = if self.chances_used > 1 { "s" } else { "" };
            if word == self.target_word {
                attron(COLOR_PAIR(GREEN));
                addstr("\nYou guessed the word! ");
                attroff(COLOR_PAIR(GREEN));
                addstr(&format!(
                    " You took {} chance{}.",
                    self.chances_used, plural
                ));
                refresh();
                quit(self);
            } else if self.chances_used > self.max_chances && word != self.target_word {
                attron(COLOR_PAIR(RED));
                addstr("\nYou Lose!");
                attroff(COLOR_PAIR(RED));
                addstr(&format!(" The word was {}", self.target_word));
                refresh();
                quit(self);
            }
        }
    }
    fn word_input(&mut self) -> Vec<String> {
        let mut letters: Vec<String> = Vec::new();
        while self.running {
            self.print_words();
            mv((self.chances_used + 1).try_into().unwrap(), 0);
            addstr(&letters.join(""));
            refresh();
            let key = getch() as u8 as char;
            if key == '\n' {
                if self.letters_typed == self.max_letters {
                    self.words_typed.push(letters.join(""));
                    self.chances_used += 1;
                    self.letters_typed = 0;
                    break;
                }
            } else if key == '\u{7f}' {
                if letters.len() > 0 {
                    letters.pop();
                    self.letters_typed -= 1;
                    mvaddstr(
                        (self.chances_used + 1).try_into().unwrap(),
                        self.letters_typed.try_into().unwrap(),
                        " ",
                    );
                }
            } else {
                if self.letters_typed < self.max_letters && key.is_alphabetic() {
                    letters.push(String::from(key.to_ascii_lowercase()));
                    self.letters_typed += 1;
                }
            }
        }
        letters
    }
    fn start(&mut self) {
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        clear();
        addstr("welcome to wordle! start typing to play.\n");
        while self.running {
            self.word_input();
        }
    }
}

fn init_game() -> Game {
    Game {
        target_word: pick_word::gen(),
        max_chances: 5,
        chances_used: 0,
        max_letters: 5,
        letters_typed: 0,
        words_typed: vec![],
        running: true,
    }
}
fn main() {
    let mut game = init_game();
    initscr();
    noecho();

    start_color();
    init_pair(NORMAL, COLOR_WHITE, COLOR_BLACK);
    init_pair(GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(RED, COLOR_RED, COLOR_BLACK);

    game.start();
}
