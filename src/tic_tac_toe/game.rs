use std::fmt::Write;
use std::string;

pub enum BoardCellState {
    Empty,
    X,
    O
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
    board_state: [[BoardCellState; 3]; 3]
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
        ]
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
