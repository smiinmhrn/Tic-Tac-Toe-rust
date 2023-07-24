use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
#[derive(PartialEq, Copy, Clone)]
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
                play_with_computer();
                break;
            }
            2 => {
                play_with_user();
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

    for i in 1..17 {
        game_board.push(GameBoardCells::BLANK);
        random_num.push(i);
    }
    
    let mut rng = thread_rng();
    random_num.shuffle(&mut rng);
    
    for _ in 0..3 {
        game_board[(random_num[0] - 1) as usize] = GameBoardCells::LOCK;
        random_num.remove(0);
    }
    
    (game_board, random_num)
}

fn print_game_board(game_board: &Vec<GameBoardCells>) {
    println!("[ Game started ]");
    for i in 0..16 {
        match game_board[i] {
            GameBoardCells::BLANK => {
                if (i + 1) < 10 {
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

fn play_with_user(){

    let mut vectors = creating_game_board();
    let mut count = 0;

    loop {
        print_game_board(&vectors.0);
        let mut result = users_move(&mut vectors.1,&mut vectors.0, 1);
        count += 1;

        if result {
            print_game_board(&vectors.0);
            println!("Player1 won!");
            break;
        }
        if count == 13 {
            print_game_board(&vectors.0);
            println!("That is a tie!");
            break;
        }

        print_game_board(&vectors.0);
        result = users_move(&mut vectors.1,&mut vectors.0, 2);
        count += 1;

        if result {
            print_game_board(&vectors.0);
            println!("Player2 won!");
            break;
        }
    }
}

fn play_with_computer(){
    let mut vectors = creating_game_board();
    let mut count = 0;

    loop {
        print_game_board(&vectors.0);
        let mut result = users_move(&mut vectors.1,&mut vectors.0, 1);
        count += 1;

        if result {
            print_game_board(&vectors.0);
            println!("Player1 won!");
            break;
        }
        if count == 13 {
            print_game_board(&vectors.0);
            println!("That is a tie!");
            break;
        }
        print_game_board(&vectors.0);
        result = computer_move(&mut vectors.1,&mut vectors.0);
        count += 1;

        if result {
            print_game_board(&vectors.0);
            println!("computer won!");
            break;
        }
    }
}

fn users_move(users_choices: &mut Vec<usize>, game_board: &mut Vec<GameBoardCells>, turn: i8) -> bool {

    println!("Player{} make your move: ", turn);
    let mut user_move = convert_string_into_int(take_input_from_user()) as usize;

    loop {
        if users_choices.contains(&user_move) {

            let index = users_choices.iter().position(|x| *x == user_move).unwrap();
            users_choices.remove(index);

            match turn {
                1 => {
                    game_board[user_move - 1] = GameBoardCells::X;
                    return winner_cheack(&game_board);
                }
                2 => {
                    game_board[user_move - 1] = GameBoardCells::O;
                    return winner_cheack(&game_board);
                }
                _ => ()
            }
        }else {
            println!("invalide choice! please try again");
            user_move = convert_string_into_int(take_input_from_user()) as usize;
        }
    }
}

fn computer_move(computer_choices: &mut Vec<usize>, game_board: &mut Vec<GameBoardCells>) -> bool{

    let computer_move = computer_choices[0];
    game_board[computer_move - 1] = GameBoardCells::O;
    computer_choices.remove(0);
    return winner_cheack(&game_board);
}

fn winner_cheack(game_board: &Vec<GameBoardCells>) -> bool {
    let mut cell_check: GameBoardCells;
    let mut repetition: i32;

    for i in 0..4 {
        repetition = 0;
        cell_check = GameBoardCells::BLANK;

        for j in 0..4 {
            let index = (i * 4) + j;

            if cell_check == game_board[index] {
                repetition +=1
            }else {
                cell_check = game_board[index];
                repetition = 1;
            }
            if cell_check != GameBoardCells::BLANK && repetition == 3 {
                return true;
            }
        }
    }

    for i in 0..4 {
        repetition = 0;
        cell_check = GameBoardCells::BLANK;

        for j in 0..4 {
            let index = (j * 4) + i;

            if cell_check == game_board[index] {
                repetition +=1
            }else {
                cell_check = game_board[index];
                repetition = 1;
            }
            if cell_check != GameBoardCells::BLANK && repetition == 3 {
                return true;
            }
        }
    }

    repetition = 0;
    cell_check = GameBoardCells::BLANK;
    for j in 0..4 {

        for i in 0..4 {
            if i == j{
                let index = i * 4 + j;

                if cell_check == game_board[index] {
                    repetition +=1;
                    
                    if cell_check != GameBoardCells::BLANK && repetition == 3 {
                        return true;
                    }
                }else {
                    cell_check = game_board[index];
                    repetition = 1;
                }
            }else {
                continue;
            }

        }
    }

    repetition = 0;
    cell_check = GameBoardCells::BLANK;
    for i in 0..4 {
        for j in 0..4 {
            if i + j == 3{
                let index = j * 4 + i;

                if cell_check == game_board[index] {
                    repetition +=1;
    
                    if cell_check != GameBoardCells::BLANK && repetition == 3 {
                        return true;
                    }
                }else {
                    cell_check = game_board[index];
                    repetition = 1;
                }
            }else {
                continue;
            }
        }
    }
    false
}