use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
enum GameBoardCells {
    X,
    O,
    BLANK,
    LOCK,
}
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

fn creating_game_board() -> (Vec<GameBoardCells>, Vec<usize>) {
    let mut game_board: Vec<GameBoardCells> = vec![];
    let mut random_num: Vec<usize> = vec![];

    for i in 0..16 {
        game_board.push(GameBoardCells::BLANK);
        random_num.push(i);
    }
    
    let mut rng = thread_rng();
    random_num.shuffle(&mut rng);
    
    for _ in 0..3 {
        game_board[random_num[0]] = GameBoardCells::LOCK;
        random_num.remove(0);
    }
    
    (game_board, random_num)
}

fn print_game_board(game_board: &Vec<GameBoardCells>) {
    for i in 0..16 {
        match game_board[i] {
            GameBoardCells::BLANK => {
                if (i+1) < 10 {
                    print!("{} |", i + 1)
                }else {
                    print!("{}|", i + 1)
                }
            }
            GameBoardCells::LOCK => print!("# |"),
            GameBoardCells::X => print!("X |"),
            GameBoardCells::O => print!("O |"),
        }

        if (i + 1) % 4 == 0 {
            println!();
        }
    }
}

fn user(){
    print_game_board(&creating_game_board().0);
}
fn computer(){
    print_game_board(&creating_game_board().0);
}