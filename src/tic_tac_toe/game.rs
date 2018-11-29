use std::fmt::Write;

#[derive(PartialEq)]
pub enum BoardCellState {
    Empty,
    X,
    O
}

#[derive(PartialEq)]
pub enum Player {
    First,
    Second
}

impl std::fmt::Display for BoardCellState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cell_repr: String;
        match self {
            BoardCellState::Empty => {
                cell_repr = String::from(" ")
            },
            BoardCellState::X => {
                cell_repr = String::from("X")
            },
            BoardCellState::O => {
                cell_repr = String::from("O")
            }
        }
        return write!(f, "{}", cell_repr)
    }
}

pub struct TicTacToe {
    board_state: [[BoardCellState; 3]; 3],
    current_player: Player
}

impl TicTacToe {
    fn is_cell_taken(&self, tic_location: usize) -> bool {
        match tic_location {
            0 ..= 2 => {
                return self.board_state[0][tic_location] != BoardCellState::Empty;
            }
            3 ..= 5 => {
                return self.board_state[1][tic_location % 3] != BoardCellState::Empty;
            }
            6 ..= 8 => {
                return self.board_state[2][tic_location % 3] != BoardCellState::Empty;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn make_move(&mut self, tic_location: usize) {
        match tic_location {
            0 ..= 2 => {
                if self.is_cell_taken(tic_location) {
                    return;
                }
                self.board_state[0][tic_location] = if self.current_player == Player::First { BoardCellState::X } else { BoardCellState::O };
            }
            3 ..= 5 => {
                if self.is_cell_taken(tic_location) {
                    return;
                }
                self.board_state[1][tic_location % 3] = if self.current_player == Player::First { BoardCellState::X } else { BoardCellState::O };
            }
            6 ..= 8 => {
                if self.is_cell_taken(tic_location) {
                    return;
                }
                self.board_state[2][tic_location % 3] = if self.current_player == Player::First { BoardCellState::X } else { BoardCellState::O };
            }
            _ => {
                ;
            }
        }

        if self.current_player == Player::First {
            self.current_player = Player::Second;
        }
        else {
            self.current_player = Player::First;
        }
    }
}

impl std::fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_repr = String::new();
        for (i, row) in self.board_state.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                write!(board_repr, " {} ", cell);

                let is_last_col = j == (row.len() - 1);
                if !is_last_col {
                    board_repr.push_str("|");
                }
            }

            let is_last_row = i == (self.board_state.len() - 1);
            if !is_last_row {
                board_repr.push_str("\n-----------\n");
            }
        }
        return write!(f, "{}", board_repr)
    }
}

pub fn new() -> TicTacToe {
    TicTacToe {
        board_state: [
            [BoardCellState::Empty, BoardCellState::Empty, BoardCellState::Empty],
            [BoardCellState::Empty, BoardCellState::Empty, BoardCellState::Empty],
            [BoardCellState::Empty, BoardCellState::Empty, BoardCellState::Empty],
        ],
        current_player: Player::First
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_board_cell_state_display_trait() {
        let should_be_x = BoardCellState::X.to_string();
        let should_be_o = BoardCellState::O.to_string();
        let should_be_empty = BoardCellState::Empty.to_string();

        assert_eq!(should_be_x, "X");
        assert_eq!(should_be_o, "O");
        assert_eq!(should_be_empty, " ");
    }
}
