use clearscreen;
use std::io::Write;

struct TicTacToe {
    slots: [[char; 3]; 3],
    rows: usize,
    cols: usize,
    winner: char,
    game_won: bool,
    is_draw: bool,
    game_over: bool,
}

impl TicTacToe {
    fn init() -> Self {
        TicTacToe {
            slots: [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']],
            rows: 3,
            cols: 3,
            winner: ' ',
            game_won: false,
            is_draw: false,
            game_over: false,
        }
    }

    fn print_board(&self) {
        clearscreen::clear().unwrap();

        for i in 0..self.rows {
            let i = i as usize;

            for j in 0..self.cols {
                let j = j as usize;
                let current = self.slots[i][j];

                if j == 2 {
                    print!("\x1b[31m {}\x1b[0m", current);
                } else {
                    let color = if current == 'X' || current == 'x' {
                        "\x1b[31m"
                    } else {
                        "\x1B[34m"
                    };
                    print!(" {} {}\x1b[0m", color, current);
                    print!("\x1B[37m|\x1b[0m");
                }
            }
            println!("");

            if i < 2 {
                println!("\x1B[37m---+---+---\x1b[0m");
            }
        }
        println!("");
    }

    fn is_row_identical(&mut self, row_index: usize) -> bool {
        let first_value = self.slots[row_index][0];

        if first_value == ' ' {
            return false;
        }

        for col in 1..self.cols {
            if self.slots[row_index][col] != first_value {
                return false;
            }
        }

        self.winner = first_value;
        true
    }

    fn is_column_identical(&mut self, column_index: usize) -> bool {
        let column_index = column_index as usize;
        let first_value = self.slots[0][column_index];

        if first_value == ' ' {
            return false;
        }

        for row in 1..self.cols {
            if self.slots[row][column_index] != first_value {
                return false;
            }
        }

        self.winner = first_value;
        true
    }

    fn is_diagonal_identical(&mut self) -> bool {
        let first_value_at_backwards_diagonal = self.slots[0][0];
        let first_value_at_forward_diagonal = self.slots[0][self.rows - 1];
        let mut is_backwards_diagonal_identical = first_value_at_backwards_diagonal != ' ';
        let mut is_forward_diagonal_identical = first_value_at_forward_diagonal != ' ';

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
                if self.slots[i][j] == ' ' {
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

    fn get_char_input(valid_chars: Vec<char>, stdin_message: &str) -> char {
        Self::get_input(valid_chars, stdin_message, false)
            .chars()
            .next()
            .expect("Empty input.")
    }

    fn trigger_prompts(&mut self) {
        let mut row: usize = 0;
        let mut col: usize = 0;

        loop {
            print!("Insert X or O: ");
            let new_char = Self::get_char_input(vec!['X', 'O'], "Invalid input. Insert X or O: ");

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

            if self.slots[row][col] == ' ' {
                self.slots[row][col] = new_char;
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

    fn reset_board(&mut self) {
        for index in 0..self.rows * self.cols {
            let i = index / self.rows;
            let j = index % self.cols;

            self.slots[i][j] = ' ';
        }
    }
}

fn main() {
    let mut game = TicTacToe::init();

    loop {
        game.print_board();
        game.trigger_prompts();

        if game.game_won {
            game.print_board();
            println!("The winner is... {}!", game.winner);
        } else if game.is_draw {
            game.print_board();
            println!("This game ended in a draw.");
        }

        if game.game_won || game.game_over {
            print!("Would you like to play again? [y/N] ");
            std::io::stdout().flush().unwrap();

            let replay = TicTacToe::get_input(
                vec!['Y', 'N'],
                "Invalid input. Please, insert only 'y' or 'n'.",
                true,
            );

            match replay.as_str() {
                "Y" => game.reset_board(),
                _ => {
                    println!("\n☆  Game Over ☆ ");
                    break;
                }
            }
        }
    }
}
