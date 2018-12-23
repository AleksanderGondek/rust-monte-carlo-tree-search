extern crate rust_mcts;

use std::io;

fn main() {
    use rust_mcts::game::Game;
    use rust_mcts::game::Player;

    let mut game_of_ttt = rust_mcts::tic_tac_toe::thing::GameOfTicTacToe::new();
    
    loop {
        print!("{}[2J", 27 as char);
        println!("{}", game_of_ttt);

        println!("Player {:?} move!", game_of_ttt.current_player());
        
        let possible_moves = game_of_ttt.possible_moves();
        println!("Possible moves are: {:?}", possible_moves);
        if possible_moves.len() <= 0 {
            println!("No more possible moves left!");
            break;
        }
        
        
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Failed to read line");
        
        match player_input {
            ref x if *x.trim() == String::from("quit") => {
                println!("Exiting, bye!");
                break;
            },
            y => {
                game_of_ttt.make_move(y);
                game_of_ttt.set_next_player();
            },
        }

        if game_of_ttt.current_player_won() {
            let mut any_input = String::new();
            println!("Player {:?} wins!", game_of_ttt.current_player());
            println!("Type anything to quit.");
            io::stdin().read_line(&mut any_input).expect("Failed to read line");
            break;
        }
    }
}
    