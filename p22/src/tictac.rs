#[derive(Debug)]
pub enum GameError {
    InvalidMove,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    InProgress,
    Winner(CellState),
    Draw,
}

pub struct TicTacToeField {
    field: [[CellState; 3]; 3],
}

pub struct Player {
    symbol: CellState,
}

impl Default for TicTacToeField {
    fn default() -> Self {
        Self::new()
    }
}

impl TicTacToeField {
    pub fn new() -> Self {
        TicTacToeField {
            field: [[CellState::Empty; 3]; 3],
        }
    }

    pub fn analyze(&self) -> GameResult {
        // Check for a winner in rows
        for i in 0..3 {
            if self.field[i][0] != CellState::Empty
                && self.field[i][0] == self.field[i][1]
                && self.field[i][1] == self.field[i][2]
            {
                println!("Winner: {:?}", self.field[i][0]);
                return GameResult::Winner(self.field[i][0]);
            }
        }

        // Check for a winner in columns
        for i in 0..3 {
            if self.field[0][i] != CellState::Empty
                && self.field[0][i] == self.field[1][i]
                && self.field[1][i] == self.field[2][i]
            {
                println!("Winner: {:?}", self.field[0][i]);
                return GameResult::Winner(self.field[0][i]);
            }
        }

        // Check diagonals
        if self.field[0][0] != CellState::Empty
            && self.field[0][0] == self.field[1][1]
            && self.field[1][1] == self.field[2][2]
        {
            println!("Winner: {:?}", self.field[0][0]);
            return GameResult::Winner(self.field[0][0]);
        }

        if self.field[0][2] != CellState::Empty
            && self.field[0][2] == self.field[1][1]
            && self.field[1][1] == self.field[2][0]
        {
            println!("Winner: {:?}", self.field[0][2]);
            return GameResult::Winner(self.field[0][2]);
        }

        // Check if the game is a draw (all cells filled)
        let mut is_full = true;
        for row in &self.field {
            for cell in row {
                if *cell == CellState::Empty {
                    is_full = false;
                    break;
                }
            }
            if !is_full {
                break;
            }
        }

        if is_full {
            return GameResult::Draw;
        }

        // Game is still in progress
        GameResult::InProgress
    }

    pub fn make_move(&self, x: u32, y: u32, player: &Player) -> Result<TicTacToeField, GameError> {
        if x > 2 || y > 2 {
            return Err(GameError::OutOfBounds);
        }
        if self.field[x as usize][y as usize] != CellState::Empty {
            return Err(GameError::InvalidMove);
        }

        let mut new_field = self.clone();
        new_field.field[x as usize][y as usize] = player.symbol;
        Ok(new_field)
    }

    // Helper method to display the current state of the board
    pub fn display(&self) {
        println!("-------------");
        for row in &self.field {
            print!("| ");
            for cell in row {
                match cell {
                    CellState::Empty => print!("  | "),
                    CellState::X => print!("X | "),
                    CellState::O => print!("O | "),
                }
            }
            println!("\n-------------");
        }
    }
}

impl Clone for TicTacToeField {
    fn clone(&self) -> Self {
        TicTacToeField { field: self.field }
    }
}

impl Player {
    pub fn new(symbol: CellState) -> Self {
        // Ensure symbol is valid (not Empty)
        let valid_symbol = match symbol {
            CellState::Empty => CellState::X, // Default to X if Empty is provided
            _ => symbol,
        };

        Player {
            symbol: valid_symbol,
        }
    }
}

// Example usage in a main function
pub fn tictactoe_example() {
    let mut game = TicTacToeField::new();
    let player1 = Player::new(CellState::X);
    let player2 = Player::new(CellState::O);

    // Example game flow
    game = game.make_move(0, 0, &player1).unwrap();
    game.display();

    game = game.make_move(1, 1, &player2).unwrap();
    game.display();

    game = game.make_move(0, 1, &player1).unwrap();
    game.display();

    game = game.make_move(2, 2, &player2).unwrap();
    game.display();

    game = game.make_move(0, 2, &player1).unwrap();
    game.display();

    let result = game.analyze();
    match result {
        GameResult::Winner(CellState::X) => println!("Player 1 wins!"),
        GameResult::Winner(CellState::O) => println!("Player 2 wins!"),
        GameResult::Draw => println!("It's a draw!"),
        GameResult::InProgress => println!("Game is still in progress."),
        _ => {}
    }
}
