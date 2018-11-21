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
        write!(f, "{}", cell_repr)
    }
}

pub fn get_greetings() -> String {
    return String::from("Hello from Rust MCTS Tic Tac Toe!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_greetings() {
        let returned_value = get_greetings();
        assert_eq!(returned_value, String::from("Hello from Rust MCTS Tic Tac Toe!"));
    }

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
