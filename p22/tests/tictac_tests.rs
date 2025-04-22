use p22::tictac::{CellState, GameResult, Player, TicTacToeField};

#[test]
fn test_new_board_is_empty() {
    let board = TicTacToeField::new();
    assert_eq!(board.analyze(), GameResult::InProgress);
}

#[test]
fn test_player_creation() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);

    // Use the players to make a move, which indirectly tests their internal state
    let board = TicTacToeField::new();
    let board = board.make_move(0, 0, &player_x).unwrap();
    let board = board.make_move(1, 1, &player_o).unwrap();

    // Game should still be in progress after two moves
    assert_eq!(board.analyze(), GameResult::InProgress);
}

#[test]
fn test_invalid_move_detection() {
    let player = Player::new(CellState::X);
    let board = TicTacToeField::new();

    // Make a valid move
    let board = board.make_move(0, 0, &player).unwrap();

    // Try to make a move on an occupied cell
    let result = board.make_move(0, 0, &player);
    assert!(result.is_err());
}

#[test]
fn test_out_of_bounds_detection() {
    let player = Player::new(CellState::X);
    let board = TicTacToeField::new();

    // Try to make a move out of bounds
    let result = board.make_move(3, 3, &player);
    assert!(result.is_err());
}

#[test]
fn test_horizontal_win() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);
    let mut board = TicTacToeField::new();

    // Player X fills the top row
    board = board.make_move(0, 0, &player_x).unwrap();
    board = board.make_move(1, 0, &player_o).unwrap(); // Player O's move
    board = board.make_move(0, 1, &player_x).unwrap();
    board = board.make_move(1, 1, &player_o).unwrap(); // Player O's move
    board = board.make_move(0, 2, &player_x).unwrap();

    // X should win with a horizontal line at the top
    assert_eq!(board.analyze(), GameResult::Winner(CellState::X));
}

#[test]
fn test_vertical_win() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);
    let mut board = TicTacToeField::new();

    // Player X fills the left column
    board = board.make_move(0, 0, &player_x).unwrap();
    board = board.make_move(1, 0, &player_o).unwrap(); // Player O's move
    board = board.make_move(1, 0, &player_x).unwrap();
    board = board.make_move(1, 1, &player_o).unwrap(); // Player O's move
    board = board.make_move(2, 0, &player_x).unwrap();

    // X should win with a vertical line on the left
    assert_eq!(board.analyze(), GameResult::Winner(CellState::X));
}

#[test]
fn test_diagonal_win() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);
    let mut board = TicTacToeField::new();

    // Player X fills the diagonal from top-left to bottom-right
    board = board.make_move(0, 0, &player_x).unwrap();
    board = board.make_move(0, 1, &player_o).unwrap(); // Player O's move
    board = board.make_move(1, 1, &player_x).unwrap();
    board = board.make_move(0, 2, &player_o).unwrap(); // Player O's move
    board = board.make_move(2, 2, &player_x).unwrap();

    // X should win with a diagonal line
    assert_eq!(board.analyze(), GameResult::Winner(CellState::X));
}

#[test]
fn test_anti_diagonal_win() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);
    let mut board = TicTacToeField::new();

    // Player X fills the diagonal from top-right to bottom-left
    board = board.make_move(0, 2, &player_x).unwrap();
    board = board.make_move(0, 0, &player_o).unwrap(); // Player O's move
    board = board.make_move(1, 1, &player_x).unwrap();
    board = board.make_move(0, 1, &player_o).unwrap(); // Player O's move
    board = board.make_move(2, 0, &player_x).unwrap();

    // X should win with an anti-diagonal line
    assert_eq!(board.analyze(), GameResult::Winner(CellState::X));
}

#[test]
fn test_draw_game() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);
    let mut board = TicTacToeField::new();

    // Fill the board in a pattern that results in a draw
    // X O X
    // X O O
    // O X X
    board = board.make_move(0, 0, &player_x).unwrap();
    board = board.make_move(0, 1, &player_o).unwrap();
    board = board.make_move(0, 2, &player_x).unwrap();

    board = board.make_move(1, 0, &player_x).unwrap();
    board = board.make_move(1, 1, &player_o).unwrap();
    board = board.make_move(1, 2, &player_o).unwrap();

    board = board.make_move(2, 0, &player_o).unwrap();
    board = board.make_move(2, 1, &player_x).unwrap();
    board = board.make_move(2, 2, &player_x).unwrap();

    // Game should be a draw
    assert_eq!(board.analyze(), GameResult::Draw);
}

#[test]
fn test_display_trait() {
    let board = TicTacToeField::new();
    let display_output = format!("{}", board);

    // Check that the display output contains expected elements
    assert!(display_output.contains("-------------"));
    assert!(display_output.contains("|"));
}

#[test]
fn test_full_game_progression() {
    let player_x = Player::new(CellState::X);
    let player_o = Player::new(CellState::O);
    let mut board = TicTacToeField::new();

    assert_eq!(board.analyze(), GameResult::InProgress);

    // Make some moves
    board = board.make_move(0, 0, &player_x).unwrap();
    assert_eq!(board.analyze(), GameResult::InProgress);

    board = board.make_move(1, 1, &player_o).unwrap();
    assert_eq!(board.analyze(), GameResult::InProgress);

    board = board.make_move(0, 1, &player_x).unwrap();
    assert_eq!(board.analyze(), GameResult::InProgress);

    board = board.make_move(2, 0, &player_o).unwrap();
    assert_eq!(board.analyze(), GameResult::InProgress);

    board = board.make_move(0, 2, &player_x).unwrap();

    // X should win with a horizontal line at the top
    assert_eq!(board.analyze(), GameResult::Winner(CellState::X));
}

#[test]
fn test_default_trait() {
    // Test that Default trait creates a new empty board
    let board = TicTacToeField::default();
    assert_eq!(board.analyze(), GameResult::InProgress);
}