extern crate rust_mcts;

use std::io;

fn main() {
    let game_of_ttt = rust_mcts::tic_tac_toe::game::new();
    
    loop {
        println!("{}", game_of_ttt);

        println!("Your move!");
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).expect("Failed to read line");
        
        match player_move {
            _ => {
                println!("Exiting!");
                break;
            }
        }
    }
}
