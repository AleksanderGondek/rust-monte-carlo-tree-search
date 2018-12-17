use super::super::game::Player; //?
use super::super::game::Game;

use std::fmt::Write;

static PLAYER_ONE_MARK: Player = Player::One(String::from("X"));
static PLAYER_TWO_MARK: Player = Player::One(String::from("O"));

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
    current_player: Player
}

impl Game for GameOfTicTacToe {
    fn current_player(&self) -> &Player {
        return &self.current_player;
    }

    fn current_player_won(&self) -> bool {
        let board = &self.board_state;
        for winning_position in WINNING_POSITIONS.iter() {
            let mut all_positions_are_a_match = true;
            for index in winning_position.iter() {
                match board[*index] {
                    Some(mark) => {
                        all_positions_are_a_match = all_positions_are_a_match && (mark == self.current_player);
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

    fn make_move(&mut self, move_to_make: String) {
        match move_to_make.parse::<usize>() {
            Ok(x) => {
                 self.board_state[x % 9] = Some(self.current_player);
            },
            Err(_) => { return; },
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