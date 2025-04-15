// First, define your custom error type
#[derive(Debug)]
pub enum GameError {
    InvalidMove,
    OutOfBounds,
}

pub struct TicTacToeField {
    field: [[char; 3]; 3],
}

pub struct Player {
    name: String,
    symbol: char,
}

impl TicTacToeField {
    pub fn new() -> Self {
        TicTacToeField {
            field: [[' '; 3]; 3],
        }
    }

    pub fn analyze(&self) -> char {
        let mut return_value: char = ' ';
        // Check for a winner
        for i in 0..3 {
            // Check rows
            if self.field[i][0] != ' ' && self.field[i][0] == self.field[i][1] && self.field[i][1] == self.field[i][2] {
                println!("Winner: {}", self.field[i][0]);
                return_value = self.field[i][0];
            }
            // Check columns
            else if self.field[0][i] != ' ' && self.field[0][i] == self.field[1][i] && self.field[1][i] == self.field[2][i] {
                println!("Winner: {}", self.field[0][i]);
                return_value = self.field[0][i];
            }
        }

        if return_value == ' ' {
            if self.field[0][0] != ' ' && self.field[0][0] == self.field[1][1] && self.field[1][1] == self.field[2][2] {
                println!("Winner: {}", self.field[0][0]);
                return_value = self.field[0][0];
            } else if self.field[0][2] != ' ' && self.field[0][2] == self.field[1][1] && self.field[1][1] == self.field[2][0] {
                println!("Winner: {}", self.field[0][2]);
                return_value = self.field[0][2];
            }
        }
        return_value
    }

    pub fn make_move(&self, x: u32, y: u32, player: &Player) -> Result<TicTacToeField, GameError> {
        if x > 2 || y > 2 {
            return Err(GameError::OutOfBounds);
        }
        if self.field[x as usize][y as usize] != ' ' {
            return Err(GameError::InvalidMove);
        }

        let mut new_field = self.clone();
        new_field.field[x as usize][y as usize] = player.symbol;
        Ok(new_field)
    }
}

impl Clone for TicTacToeField {
    fn clone(&self) -> Self {
        TicTacToeField {
            field: self.field.clone(),
        }
    }
}