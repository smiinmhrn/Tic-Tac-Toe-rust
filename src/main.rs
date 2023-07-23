use std::io;
fn main() {
    start_menu();
}

fn start_menu() {
    println!("[ Welcome to the tic tac toe game ]");
    println!("Choose the game mode");
    println!("1. Playing with computer");
    println!("2. Playing with other user");
    
    let mut user_choice = convert_string_into_int(take_input_from_user());

    loop {
        match user_choice {
            1 => {
                computer();
                break;
            }
            2 => {
                user();
                break;
            }
            _ => {
                println!("wrong command please try again");
                user_choice = convert_string_into_int(take_input_from_user());
            }

        }
    }
}

fn take_input_from_user() -> String {
    let mut user_input = String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to reading the input");
    user_input
}

fn convert_string_into_int(string_to_convert: String) -> i32 {
    let string: i32 = match  string_to_convert.trim().parse(){
        Ok(num) => num,
        Err(_) => -1,
    };
    string
}

fn user(){}
fn computer(){}