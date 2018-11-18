use std::string;

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
}
