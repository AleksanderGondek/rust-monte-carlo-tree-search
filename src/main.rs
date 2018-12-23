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
        
        let possible_moves = game_of_ttt.get_possible_moves();
        println!("Possible moves are: {:?}", possible_moves);
        if possible_moves.len() <= 0 {
            println!("No more possible moves left!");
            break;
        }
        
        
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).expect("Failed to read line");
        
        let possible_field_number: Result<usize, std::num::ParseIntError> = player_move.trim().parse(); 
        match possible_field_number {
            Ok(x @ 0 ..= 8) => {                
                game_of_ttt.make_move(x);
                game_of_ttt.set_next_player();
            },
            Ok(_) => println!("Please provide number from range <1,8> or type 'quit' to exit."),
            Err(_) => {
                match player_move.trim().as_ref() {
                    "quit" => {
                        println!("Exiting, bye!");
                        break;
                    }
                    _ => println!("Invalid command. Type 'quit' to exit."),
                }
            },
        }

        if game_of_ttt.current_player_won() {
            println!("Player {:?} wins!", game_of_ttt.current_player());
            println!("Type anything to quit.");
            io::stdin().read_line(&mut player_move).expect("Failed to read line");
            break;
        }
    }
}
    