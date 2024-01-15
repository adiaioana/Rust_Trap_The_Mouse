mod board;
use std::io;
use std::thread::current;
use macroquad::prelude::*;
use board::Board;

fn Player2(GB:&mut Board){
    loop {
        println!("[2] Choose where to move mouse> (1/2/3/4/5/6)");
        let mut game_option=1;
        let mut input_text = String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => game_option=i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        if (*GB).move_mouse(game_option)==true {
            break;
        }
    }
    
    /* 
        while !is_key_pressed(KeyCode::Enter) {
            if is_key_pressed(KeyCode::Key1) {
                game_option=1;
            }
            else if is_key_pressed(KeyCode::Key2) {
                game_option=2;
            }
            else if is_key_pressed(KeyCode::Key3) {
                game_option=3;
            }
            else if is_key_pressed(KeyCode::Key4) {
                game_option=4;
            }
            else if is_key_pressed(KeyCode::Key5) {
                game_option=5;
            }
            else if is_key_pressed(KeyCode::Key6) {
                game_option=6;
            }
        }*/
       
}

fn Player1(GB:&mut Board){
    loop {
        let mut x=0;
        let mut y=0;
        println!("[1] Choose where to place wall> X=");
        let mut input_text = String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => {x=i;},
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        println!("\nY=");
        let mut input_text = String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => {y=i;},
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        if (*GB).make_wall((x,y))==true {
            break;
        }
    }
}
fn main() {
    let mut game_board:Board=Board::new();
    let mut current_turn:i32=2;
    while game_board.state(current_turn)==0 {
        current_turn=3-current_turn;
        
        println!("It is now the turn for Player {}", current_turn);
        game_board.print_for_debug();
        if current_turn==2 {
            Player2(&mut game_board);
        }
        else {
            Player1(&mut game_board);
            //Player1(GB:&mut game_board);
        }
    }
    println!("Player {} won!", current_turn);
}
