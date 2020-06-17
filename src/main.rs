use std::io;
use std::cmp;

/*
    A big thanks for DirtGrubDylan for helping me with this project to learn more about rust and what I can do with it!
    Link to his github -> https://github.com/DirtGrubDylan/tic_tac_toe/blob/master/src/game.rs
    Thank you DirtGrubDylan, looking at your code helped me a lot, and I added some of my own features!
*/

// Board is a Vec<String> inside another Vec
type Board = Vec<Vec<char>>;
// Board Size which is 3
const BOARD_SIZE: usize = 3;

enum Turn {
    // Player Turn
    Player,
    // Bot Turn
    Bot
}

struct Game {
    // Game Board
    board: Board,
    // Current Turn
    turn: Turn,
    // Player Piece ('X' or 'O')
    player: char,
    // Bot Piece ('X' or 'O')
    bot: char,
    // If bot is maximizing or minimizing
    bot_maximizing: bool,

}

// Impl of Game
impl Game {
    
    // Creates a new game, and returns Game Struct
    pub fn new() -> Game {
        Game {
            board: vec![vec!['-'; BOARD_SIZE]; BOARD_SIZE],
            turn: Turn::Player,
            player: '-',
            bot: '-',
            bot_maximizing: false,
        }
    }

    // This plays the whole game
    fn play_game(&mut self) {
        // Gets the users piece
        self.get_piece();

        // Will print and example board showing all the moves you can make
        self.example_board();

        let mut finished = false;
        while !finished {
            // Gets the move from the Bot or the Player
            self.play_turn();
            // Prints the board
            self.print_board();

            // Checks if anyone has won, and if so will print who won, and will then restart
            if self.check_win() {
                finished = true;
                match self.turn {
                    Turn::Player => {
                        println!("Player wins!");
                        break;
                    },
                    Turn::Bot => {
                        println!("Bot wins!");
                        break;
                    }
                }
            }

            // Checks if there are any open moves
            if !self.open_moves() {
                finished = true;
                println!("Tie!");
                break;
            }

            // Changes whose turn it is
            self.next_turn();
        }

        // Will check if the play wants to keep playing
        if finished {
            self.player_finished();
        }
    }

    // This gets the piece the player wants ('X' or 'O')
    fn get_piece(&mut self) {
        let mut player_input = String::new();
        loop {
            println!("\nDo you want to be 'X' or 'O'?");
            match io::stdin().read_line(&mut player_input) {
                Err(_) => {
                    println!("\nError reading input. Try again!");
                    continue;
                },
                Ok(_) => {
                    player_input = player_input.trim().to_uppercase();
                    if !player_input.eq("X") && !player_input.eq("O") {
                        println!("\nPlease enter 'X' or 'O'!");
                        continue;
                    }

                    let chars: Vec<char> = player_input.chars().collect();
                    self.player = chars[0];

                    self.turn = if self.player == 'X' { Turn::Player } else { Turn::Bot };
                    self.bot_maximizing = self.player != 'X';

                    break;
                }
            }
        }
    }

    // Will get the bots piece, and will play whoever turn it is.
    fn play_turn(&mut self) {
        self.bot = if self.player == 'X' { 'O' } else { 'X' };

        match self.turn {
            Turn::Player => {
                let num = self.get_player_move();
                let location = self.get_move(num);
                self.board[location.0][location.1] = self.player;
            },
            Turn::Bot => self.get_bot_move(),
        };
    }

    // Switches the turn of the bot or player
    fn next_turn(&mut self) {
        self.turn = match self.turn {
            Turn::Player => Turn::Bot,
            Turn::Bot => Turn::Player,
        };
    }

    // This will print the 3x3 board cleanly, and neatly
    fn print_board(&self) {
        let seperator = "+---+---+---+";

        println!("\n{}", seperator);

        for row in &self.board {
            let new_row: Vec<String> = row.iter().map(|c| c.to_string()).collect();
            println!("| {} |\n{}", new_row.join(" | "), seperator);
        }

        print!("\n\n");
    }

    // Prints a board with the numbers, and shows the position of the numbers
    fn example_board(&self) {
        let seperator = "+---+---+---+";
        println!("\n{}", seperator);
        let example_board: Board = vec![
                                vec!['1', '2', '3'], 
                                vec!['4', '5', '6'],
                                vec!['7', '8', '9'],
                            ];

        for row in &example_board {
            let new_row: Vec<String> = row.iter().map(|c| c.to_string()).collect();
            println!("| {} |\n{}", new_row.join(" | "), seperator);
        }

        print!("\n\n");
    }

    // Will get the move the player wants, and checks for errors
    fn get_player_move(&self) -> u32 {
        loop {
            println!("\nEnter a number between 1 - 9!");
            let mut player_move = String::new();
            io::stdin()
                .read_line(&mut player_move)
                .expect("\nFailed to read player move.\nTry again!\n");
            
            let player_move: u32 = match player_move.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("\nPut a valid unsigned integer!");
                    continue;
                },
            };

            if player_move < 1 || player_move > 9 {
                println!("\nYou must enter a number 1 - 9!");
                continue;
            }

            if !self.check_move(player_move) {
                println!("\nSpot filled. Try again!");
                continue;
            }

            return player_move;
        }
    }

    // This will get the bot move, by using the minimax algorithm!
    fn get_bot_move(&mut self) { 
        let mut best_row = 0;
        let mut best_col = 0;
        let mut best_val_x = -10000;
        let mut best_val_o = 10000;

        if self.player == 'X' {
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == '-' {
                        self.board[i][j] = self.bot;
                        let move_val = self.minimax(0, true);
                        self.board[i][j] = '-';
    
                        if move_val < best_val_o {
                            best_row = i;
                            best_col = j;
                            best_val_o = move_val;
                        }
                    }
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == '-' {
                        self.board[i][j] = self.bot;
                        let move_val = self.minimax(0, false);
                        self.board[i][j] = '-';
    
                        if move_val > best_val_x {
                            best_row = i;
                            best_col = j;
                            best_val_x = move_val;
                        }
                    }
                }
            }
        }

        self.board[best_row][best_col] = self.bot;
    }

    fn minimax(&mut self, depth: i32, maximizing_player: bool) -> i32 {
        let score = self.evaluate();

        if score == 10 { return score }
        if score == -10 { return score }
        if score == 0 { return score }

        if maximizing_player {
            let mut max_eval = -10000;
            for i in 0..3{
                for j in 0..3 {
                    if self.board[i][j] == '-' {
                        if self.player == 'X' {
                            self.board[i][j] = self.player;
                        } else {
                            self.board[i][j] = self.bot;
                        }
                        max_eval = cmp::max(max_eval, self.minimax(depth+1, !maximizing_player));
                        self.board[i][j] = '-';
                    }
                }
            }

            return max_eval
        } else {
            let mut min_eval = 10000;
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == '-' {
                        if self.player == 'X' {
                            self.board[i][j] = self.bot;
                        } else {
                            self.board[i][j] = self.player;
                        }
                        min_eval = cmp::min(min_eval, self.minimax(depth+1, !maximizing_player));
                        self.board[i][j] = '-';
                    }
                }
            }

            return min_eval
        }
    }

    fn evaluate(&self) -> i32 {

        // This for loop evaluates the Columns, and Rows to see if anyone wins.
        // It will return a negative or postive 10 based on who is maximizing, and minimizing.
        for index in 0..3 {
            if self.board[index][0] == self.board[index][1] && self.board[index][1] == self.board[index][2] && self.board[index][0] != '-' {
                if self.bot_maximizing {
                    if self.board[index][1] == self.bot {
                        return 10
                    }

                    return -10
                } else {
                    if self.board[index][1] == self.bot {
                        return -10
                    }

                    return 10
                }
            }

            if self.board[0][index] == self.board[1][index] && self.board[1][index] == self.board[2][index] && self.board[0][index] != '-' {
                if self.bot_maximizing {
                    if self.board[1][index] == self.bot {
                        return 10
                    }

                    return -10
                } else {
                    if self.board[1][index] == self.bot {
                        return -10
                    }

                    return 10
                }
            }
        }

        // Evaluate Diagonal
        // +---+---+---+
        // | X | - | - |
        // +---+---+---+
        // | - | X | - |
        // +---+---+---+
        // | - | - | X |
        // +---+---+---+
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != '-' {
            if self.bot_maximizing {
                if self.board[1][1] == self.bot {
                    return 10
                }

                return -10
            } else {
                if self.board[1][1] == self.bot {
                    return -10
                }

                return 10
            }
        }

        // Evaluate Diagonal
        // +---+---+---+
        // | - | - | X |
        // +---+---+---+
        // | - | X | - |
        // +---+---+---+
        // | X | - | - |
        // +---+---+---+
        if self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2] && self.board[2][0] != '-' {
            if self.bot_maximizing {
                if self.board[1][1] == self.bot {
                    return 10
                }

                return -10
            } else {
                if self.board[1][1] == self.bot {
                    return -10
                }

                return 10
            }
        }

        // If no open moves, then it's a tie, so return 0, so no score for either.
        if !self.open_moves() {
            return 0
        }

        -1
    }

    // This will make sure a move is valid
    fn check_move(&self, player_move: u32) -> bool {
        let temp_location = self.get_move(player_move);
        
        match self.board[temp_location.0][temp_location.1] {
            'X' | 'O' => false,
            _ => true,
        }
    }

    // This will get Row and Column of the number entered for the board
    fn get_move(&self, board_move: u32) -> (usize, usize) {
        let row = (board_move - 1) / 3;
        let col = (board_move - 1) % 3;

        (row as usize, col as usize)
    }

    // This will check if there are any winners by looking at Columns, Rows, and Diagonials
    fn check_win(&self) -> bool {
        let mut same_row = false;
        let mut same_col = false;

        for index in 0..3 {
            same_row |= self.board[index][0] == self.board[index][1] && self.board[index][1] == self.board[index][2] && self.board[index][0] != '-';
            same_col |= self.board[0][index] == self.board[1][index] && self.board[1][index] == self.board[2][index] && self.board[0][index] != '-';
        }

        let diagional_1 = self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != '-';
        let diagional_2 = self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2] && self.board[2][0] != '-';

        same_col || same_row || diagional_1 || diagional_2
    }

    // Looks for any open moves
    fn open_moves(&self) -> bool {
        for row in &self.board {
            for item in row {
                if *item == '-' {
                    return true
                }
            }
        }

        false
    }

    // Check if the player is finished, and wants to keep playing
    fn player_finished(&self) {
        println!("\nDo you want to play again? (y) or (n)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("\nFailed to read.\nTry again!\n");

        if play_again.trim().to_lowercase().eq("y") {
            self.reset();
        } else {
            println!("\nSorry to see you go... Goodbye!");
        }
    }

    // Starts a new game
    fn reset(&self) {
        let mut game = Game::new();
        game.play_game();
    }
}

// Starts a game
fn tic_tac_toe() {
    let mut game = Game::new();
    game.play_game();
}

fn main() {
    tic_tac_toe();
}
