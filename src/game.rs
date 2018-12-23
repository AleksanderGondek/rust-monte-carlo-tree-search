#[derive(Clone, Debug, PartialEq)]
pub enum Player {
    One(String),
    Two(String),
}

pub trait Game: std::fmt::Display {
    fn current_player(&self) -> &Player;
    fn current_player_won(&self) -> bool;
    fn make_move(&mut self, move_to_make: String) -> bool;
    fn possible_moves(&self) -> Vec<String>;
    fn set_next_player(&mut self);    
}
