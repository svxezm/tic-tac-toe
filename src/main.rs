use clearscreen;
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Empty,
    X,
    O,
}

enum GameState {
    Replay,
    Exit,
}

impl Slot {
    fn to_char(slot: Slot) -> char {
        match slot {
            Slot::X => 'X',
            _ => 'O',
        }
    }
}

struct TicTacToe {
    slots: [[Slot; 3]; 3],
    rows: usize,
    cols: usize,
    winner: Slot,
    game_won: bool,
    is_draw: bool,
    game_over: bool,
}

impl TicTacToe {
    fn play() {
        loop {
            match Self::init() {
                GameState::Replay => continue,
                GameState::Exit => break,
            }
        }
    }

    fn init() -> GameState {
        let mut game = Self {
            slots: [[Slot::Empty; 3]; 3],
            rows: 3,
            cols: 3,
            winner: Slot::Empty,
            game_won: false,
            is_draw: false,
            game_over: false,
        };

        print!("Which symbol would you like to use first? (X or O) ");
        let mut current_symbol =
            Self::get_slot_input(vec!['X', 'O'], "Invalid input. Insert X or O: ");

        loop {
            clearscreen::clear().unwrap();
            println!(
                "Current turn: \x1b[33m{}\x1b[0m\n",
                Slot::to_char(current_symbol)
            );

            game.print_board();
            game.trigger_prompts(current_symbol);

            if game.game_won {
                clearscreen::clear().unwrap();
                game.print_board();
                println!("The winner is... {}!", Slot::to_char(game.winner));
            } else if game.is_draw {
                clearscreen::clear().unwrap();
                game.print_board();
                println!("This game ended in a draw.");
            }

            if game.game_won || game.is_draw {
                print!("Would you like to play again? [y/N] ");
                std::io::stdout().flush().unwrap();

                let replay = TicTacToe::get_input(
                    vec!['Y', 'N'],
                    "Invalid input. Please, insert only 'y' or 'n'.",
                    true,
                );

                match replay.as_str() {
                    "Y" => return GameState::Replay,
                    _ => {
                        println!("\n☆  Game Over ☆ ");
                        return GameState::Exit;
                    }
                }
            }

            if !game.game_over {
                current_symbol = match current_symbol {
                    Slot::X => Slot::O,
                    _ => Slot::X,
                };
            }
        }
    }

    fn print_board(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let slot = match self.slots[i][j] {
                    Slot::X => "\x1b[31mX\x1b[0m",
                    Slot::O => "\x1b[34mO\x1b[0m",
                    Slot::Empty => " ",
                };

                print!(" {} ", slot);

                if j < 2 {
                    print!("\x1B[37m|\x1b[0m");
                }
            }
            println!();

            if i < 2 {
                println!("\x1B[37m---+---+---\x1b[0m");
            }
        }
        println!();
    }

    fn is_row_identical(&mut self, row: usize) -> bool {
        let first = self.slots[row][0];

        if first == Slot::Empty {
            return false;
        }

        if self.slots[row].iter().all(|&s| s == first) {
            self.winner = first;
            true
        } else {
            false
        }
    }

    fn is_column_identical(&mut self, col: usize) -> bool {
        let first = self.slots[0][col];

        if first == Slot::Empty {
            return false;
        }

        if self.slots.iter().all(|&s| s[col] == first) {
            self.winner = first;
            true
        } else {
            false
        }
    }

    fn is_diagonal_identical(&mut self) -> bool {
        let first_value_at_backwards_diagonal = self.slots[0][0];
        let first_value_at_forward_diagonal = self.slots[0][self.rows - 1];
        let mut is_backwards_diagonal_identical = first_value_at_backwards_diagonal != Slot::Empty;
        let mut is_forward_diagonal_identical = first_value_at_forward_diagonal != Slot::Empty;

        for i in 1..self.rows {
            if self.slots[i][i] != first_value_at_backwards_diagonal {
                is_backwards_diagonal_identical = false;
            }
            if self.slots[i][i] != first_value_at_forward_diagonal {
                is_forward_diagonal_identical = false;
            }
        }

        if is_backwards_diagonal_identical {
            self.winner = first_value_at_backwards_diagonal;
        } else if is_forward_diagonal_identical {
            self.winner = first_value_at_forward_diagonal;
        }

        is_backwards_diagonal_identical || is_forward_diagonal_identical
    }

    fn is_board_full(&self) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.slots[i][j] == Slot::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn get_input(valid_chars: Vec<char>, stdin_message: &str, allow_empty: bool) -> String {
        std::io::stdout().flush().unwrap();
        let mut input = String::new();

        loop {
            std::io::stdin().read_line(&mut input).expect(stdin_message);

            if allow_empty == true && input.trim().is_empty() {
                return String::from("");
            }

            let chosen_char = input
                .trim()
                .to_uppercase()
                .chars()
                .next()
                .expect("Empty input.");

            match valid_chars.contains(&chosen_char) {
                true => return String::from(chosen_char),
                false => {}
            }
        }
    }

    fn get_slot_input(valid_chars: Vec<char>, stdin_message: &str) -> Slot {
        let input = Self::get_input(valid_chars, stdin_message, false);

        match input.to_uppercase().trim() {
            "X" => Slot::X,
            "O" => Slot::O,
            _ => Slot::Empty,
        }
    }

    fn trigger_prompts(&mut self, symbol: Slot) {
        let mut row: usize;
        let mut col: usize;

        loop {
            print!("Insert the row: ");
            row = Self::get_input(
                vec!['1', '2', '3'],
                "Invalid input. Insert the row (1, 2, or 3): ",
                false,
            )
            .parse()
            .unwrap();
            row -= 1;

            print!("Insert the column: ");
            col = Self::get_input(
                vec!['1', '2', '3'],
                "Invalid input. Insert the row (1, 2, or 3): ",
                false,
            )
            .parse()
            .unwrap();
            col -= 1;

            if self.slots[row][col] == Slot::Empty {
                self.slots[row][col] = symbol;
                break;
            } else {
                println!("That slot is already filled! Try again.");
            }

            if self.game_won || self.is_draw {
                self.game_over = true;
                break;
            }
        }

        self.game_won = self.is_row_identical(row)
            || self.is_column_identical(col)
            || self.is_diagonal_identical();
        self.is_draw = !self.game_won && self.is_board_full();
    }
}

fn main() {
    TicTacToe::play();
}
