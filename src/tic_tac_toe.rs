use std::fmt::Write;

use super::game::Player;
use super::game::Game;

static WINNING_POSITIONS: [[usize; 3]; 8] = [
    [0,1,2],
    [3,4,5],
    [6,7,8],
    [0,3,6],
    [1,4,7],
    [2,5,8],
    [0,4,8],
    [2,4,6],
];

pub struct GameOfTicTacToe {
    board_state: [Option<Player>; 9],
    current_player_index: usize,
    players: Vec<Player>,
}

impl GameOfTicTacToe {
    pub fn new() -> GameOfTicTacToe {
        GameOfTicTacToe {
            board_state: [None, None, None, None, None, None, None, None, None],
            current_player_index: 0,
            players: vec![
                Player::One(String::from("O")),
                Player::One(String::from("X")),
            ],
        }
    }
}

impl Game for GameOfTicTacToe {
    fn current_player(&self) -> &Player {
        return &self.players[self.current_player_index];
    }

    fn current_player_won(&self) -> bool {
        let board = &self.board_state;
        for winning_position in WINNING_POSITIONS.iter() {
            let mut all_positions_are_a_match = true;
            for index in winning_position.iter() {
                match board[*index] {
                    Some(ref mark) => {
                        all_positions_are_a_match = all_positions_are_a_match && (*mark == *self.current_player());
                    },
                    _ => { all_positions_are_a_match = false; break; },
                }
            }

            if all_positions_are_a_match {
                return true;
            }
        }

        return false;
    }

    fn make_move(&mut self, move_to_make: String) -> bool {
        match move_to_make.trim().parse::<usize>() {
            Ok(x) => {
                let target_index = x % 9;
                if self.board_state[target_index].is_some() {
                    return false;
                }

                self.board_state[x % 9] = Some(self.current_player().clone());
            },
            Err(_) => { return false; },
        }

        return true;
    }

    fn possible_moves(&self) -> Vec<String> {
        let mut moves = Vec::new();
        for (i, cell) in self.board_state.iter().enumerate() {
            match cell {
                Some(_) => continue,
                _ => moves.push(i.to_string()),
            }
        }
        return moves;
    }

    fn set_next_player(&mut self) {
        match self.current_player_index {
            i if i < (self.players.len() - 1) => {
                self.current_player_index += 1;
            },
            i if i == (self.players.len() - 1) => {
                self.current_player_index = 0;
            },
            _ => { ; }
        }
    }

}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::One(name) => { 
                write!(f, "{}", name)
            },
            Player::Two(name) => {
                write!(f, "{}", name)
            }
        }
    }
}

impl std::fmt::Display for GameOfTicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_repr = String::new();

        for (i, cell) in self.board_state.iter().enumerate() {
            match cell {
                Some(player) => {
                    write!(board_repr, " {} ", player);
                },
                _ => {
                    write!(board_repr, "   ");
                }
            }

            let is_last_col = i % 3 == 2;
            if !is_last_col {
                board_repr.push_str("|");
            }

            match i {
                2 | 5 => { board_repr.push_str("\n-----------\n"); },
                8 => { board_repr.push_str("\n"); },
                _ => { ; },
            }
        }

        return write!(f, "{}", board_repr)
    }
}