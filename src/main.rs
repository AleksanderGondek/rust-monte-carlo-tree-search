extern crate rust_mcts;

fn main() {
    let game_of_ttt = rust_mcts::tic_tac_toe::game::new();
    println!("{}", game_of_ttt);
}
