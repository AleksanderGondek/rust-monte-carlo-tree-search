use super::super::game::Player; //?
use super::super::game::Game;

use std::fmt::Write;

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
                    _ => { ; },
                }
            }

            if all_positions_are_a_match {
                return true;
            }
        }

        return false;
    }

    fn make_move(&mut self, move_to_make: String) {
        match move_to_make.parse::<usize>() {
            Ok(x) => {
                 self.board_state[x % 9] = Some(self.current_player().clone());
            },
            Err(_) => { return; },
        }
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
            i if i <= (self.players.len() - 1) => {
                self.current_player_index += 1;
            },
            i if i == self.players.len() => {
                self.current_player_index = 0;
            },
            _ => { ; }
        }
    }

}

impl std::fmt::Display for GameOfTicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_repr = String::new();

        for (i, cell) in self.board_state.iter().enumerate() {
            write!(board_repr, " {:?} ", cell); 

            let is_last_col = (i % 3 == 0) && i != 0;
            if !is_last_col {
                board_repr.push_str("|");
            }

            match i {
                2 | 5 | 8 => { board_repr.push_str("\n-----------\n"); },
                _ => { ; },
            }
        }

        return write!(f, "{}", board_repr)
    }
}