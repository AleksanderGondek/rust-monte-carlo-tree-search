extern crate rust_mcts;

use std::io;

fn main() {
    let mut game_of_ttt = rust_mcts::tic_tac_toe::game::new();
    
    loop {
        println!("{}", game_of_ttt);

        println!("Your move!");
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).expect("Failed to read line");
        
        let possible_field_number: Result<usize, std::num::ParseIntError> = player_move.trim().parse(); 
        match possible_field_number {
            Ok(0) => println!("Please provide number from range <1,9>."),
            Ok(x @ 1 ..= 9) => {                
                game_of_ttt.make_move(x - 1);
            },
            Ok(_) => println!("Please provide number from range <1,9> or type 'quit' to exit."),
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
    }
}
    