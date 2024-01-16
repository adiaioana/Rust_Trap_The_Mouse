mod board;
mod prereq;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};
use macroquad::prelude::*;
use std::str;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use board::{Board,gameboard_state, any_difference};
use prereq::BasicMessages;
#[derive(Clone)]
struct Room {
    room_nb: i32,
    room_name: String,
    player1: i32,
    player2: i32,
    game_state: i32,
    game_board: Board
}
impl Room{
    pub fn new() -> Self {
        Room{room_nb:0, room_name:String::from(" "),player1:0,player2:0,game_state:0, game_board:Board::new()}
    }
}

type SharedState = Arc<Mutex<HashMap<String, Room>>>;
///
fn room_handling_for_client( buffer:[u8;1024], state: SharedState, idclient:i32) -> String{ 
    let cpy_buff=str::from_utf8(&buffer).unwrap();
    let recv_mess=parse_message(&cpy_buff);
    let arg1:&str=&recv_mess.0.as_str();
    let arg2:&str=&recv_mess.1.as_str();
    let mess=process_room_request(arg1, arg2, &state, idclient);
    mess
}
fn get_client_option(stream: &mut TcpStream) -> char {
    let mut buffer = [0; 1024];
    let prompt = "\nDo you want to wait for another player(W) or play with the computer(C)? [W/C]\n";
    stream.write_all(prompt.as_bytes()).expect("Failed to write to stream");

    let mut option = 'N';

    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            continue;
        }

        let input = match str::from_utf8(&buffer[..bytes_read]) {
            Ok(v) => v.trim(),
            Err(_) => {
                continue;
            },
        };

        if input.contains('W') {
            option = 'W';
            break;
        } else if input.contains('C') {
            option = 'C';
            break;
            } else {
            let message = "Please enter option (W - Waiting for another player, C - Play with computer) [W/C]:";
            stream.write_all(message.as_bytes()).expect("Failed to write to stream");
            }
            buffer = [0; 1024];  // Clear the buffer for the next read
        }
        
        option
    }


fn handle_client(mut stream: TcpStream, state: SharedState, idclient:i32) {
    //Step 1: Join a room
    let mut type_of_pawn=0;
    let mut buffer = [0; 1024]; let mut whichroom=String::from("");
    loop{
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        if bytes_read==0 {
            break;
        }
        let mess=room_handling_for_client(buffer, state.clone(), idclient);
        if mess.contains("Succesfully joined") {
            
            let mut ind:i32=0;
            for word in mess.split_whitespace() {
                ind=ind+1;
                if ind>3 && word.len()>0 { //"Succesfully joined room:"
                    whichroom=word.to_string();
                    break;
                }
            }
            
            break;
        }
        stream.write_all(mess.as_bytes()).expect("Failed to write to stream");
        buffer=[0; 1024];  //Sending room entering status
    }   
    let mut playing_with_computer=0;
    //Wait/Play(w Player/Computer)
    let gs=game_state(&whichroom, &state);
    if gs==0 {
        //Option to wait or to play with computer
        let client_option= get_client_option(&mut stream);
        if client_option=='W' {
            let mut begin=0;
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => begin= n.as_secs(),
                Err(_) => panic!("SystemTime not working!"),
            }
            loop {
                let gs = game_state(&whichroom, &state);
                if gs == 1 {
                    let message = "Another player joined. Game starts now.\n";
                    stream.write_all(message.as_bytes()).expect("Failed to write to stream");
                    break;
                }
        
                let time_now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => n.as_secs(),
                    Err(_) => panic!("SystemTime not working!"),
                };
        
                if time_now - begin > 300 {
                    let message = "Waited too long. Ending application\n";
                    stream.write_all(message.as_bytes()).expect("Failed to write to stream");
                    return;
                }
        
                // Sleep for a short duration to prevent the loop from consuming too much CPU
                thread::sleep(Duration::from_millis(100));
            }
        }
        else {
            playing_with_computer=1;
        }
        type_of_pawn=1; //Wall
    }
    else{
        //Now, it's game time :0
        type_of_pawn=2; //Mouse
        let mess="Game time\n";
        stream.write_all(mess.as_bytes()).expect("Failed to write to stream");
    }

    let output=BasicMessages::new();
    let mut game_board:Board=Board::new();//game_board should be the one from the Rooms struct 
    let mut game_board_cache=Board::new();
    let mut the_end=false;
    let turns:Vec<char>=vec!['-','W','M'];
    if type_of_pawn==1 {
        game_board_cache.make_wall((1,1));
    }
    loop {
        if the_end {
            break;
        }
        let game_aux:Board=game_board.clone();
        let ifwinner=gameboard_state(&game_aux);
        match ifwinner {
            'W' => {
                the_end=true;
                if type_of_pawn==2 {
                    stream.write_all(output.lostmess().as_bytes()).expect("Failed to write to stream");
                }
                else {
                    stream.write_all(output.wonmess().as_bytes()).expect("Failed to write to stream");
                }
            }
            'M' => {
                the_end=true;
                if type_of_pawn==1 {
                    stream.write_all(output.lostmess().as_bytes()).expect("Failed to write to stream");
                }
                else {
                    stream.write_all(output.wonmess().as_bytes()).expect("Failed to write to stream");
                }
            }
            _ =>{
                //flag: add mutex
                if any_difference(&game_board, &game_board_cache) {
                    //means a move was done, so amazing
                    if type_of_pawn ==2 {
                        stream.write_all(output.movemousemess().as_bytes()).expect("Failed to write to stream");
                    }
                    else{
                        stream.write_all(output.placewallmess().as_bytes()).expect("Failed to write to stream");
                    }
                    buffer=[0;1024];
                    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
                    if bytes_read==0 {
                        println!("Error: not reading move");
                    }
                    //update moves string 
                    let mut ind:i32=0;
                    let mut var:Vec<i32>=Vec::new();
                    for word in str::from_utf8(&buffer).unwrap().split_whitespace() {
                        ind=ind+1;
                        var.push(word.parse().unwrap());
                    }

                    if type_of_pawn ==2{
                        game_board.move_mouse(var[0]);
                    }
                    else {
                        game_board.make_wall((var[0],var[1]));
                    }
                }
                stream.write_all(game_board.translate_to_moves(turns[type_of_pawn]).as_bytes()).expect("Failed to write to stream");
                if playing_with_computer!=0 {
                    //Computer makes random move
                }
                game_board_cache=game_board.clone();
            }
        }

    }
    
}

fn main() {
    let mut no_clients=0;
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    let state = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = state.clone();
                no_clients=no_clients+1;
                thread::spawn(move || {
                    handle_client(stream, state_clone,no_clients);
                });
            }
            Err(e) => eprintln!("Failed to establish a connection: {}", e),
        }
        }
        }
        fn game_state(room_name: &str, state: &SharedState)->i32 {
            let rooms = state.lock().unwrap();
            for it in rooms.iter() {
                if it.0.contains(room_name) {
                    return it.1.game_state;
                }
            }
            return -1;
        }
        fn parse_message(msg: &str) -> (String, String) { // verified
            
            if (&msg).contains("create") {
                let mut whichroom=String::from("auleu");
                for word in msg.split_whitespace(){

                    if !(&word).contains("create") && word.len()>0 {
                        whichroom=word.to_string();
                        break;
                    }
                }
                return (String::from("create"),whichroom);
            }
            else if (&msg).contains("join") {
                let mut whichroom=String::from("auleu");
                for word in msg.split_whitespace() {
                    
                    if word!="join" && word.len()>0 {
                        whichroom=word.to_string();
                        break;
                    }
                }
                return (String::from("join"), whichroom);
            }
            else if(&msg).contains("print") {
                return (String::from("print"),String::from("-"));
            }
            return (String::from("invalid"), String::from("-"));
        
        }
        fn print_all_rooms(rooms:&HashMap<String,Room>) {
            for roomy in rooms.iter() {
                println!("Room with name {} has id {} with player1={},player2={} and game_state={}", 
                roomy.0,roomy.1.room_nb, roomy.1.player1,roomy.1.player2,roomy.1.game_state);
            }
        }
        fn process_room_request(command: &str, room_name: &str, state: &SharedState, idc: i32) -> String{
        // proceseaza requesturile legate de camere
        let mut rooms = state.lock().unwrap();
        match command {
            "create" => {
                // Create a new room and add it to the global state
                let lg=rooms.len();
                let a: Room=Room{room_nb:lg as i32, room_name:String::from(room_name),player1:0,player2:0,game_state:0, game_board:Board::new()};
                rooms.insert(room_name.to_string(),a);
                let ans=String::from("Succesfully created room: ");
                return ans+room_name;
            }
            "join" => {
                let mut ok:bool=false;
                let mut a:Room=Room::new();
                for it in rooms.iter() {
                    if it.0.contains(room_name) {
                        ok=true;
                        a=it.1.clone();
                        if a.player1==0 {
                            a.player1=idc;
                        }
                        else{
                            a.player2=idc;
                            a.game_state=1;
                        }
                        break;
                    }
                }
                // verifica daca exista camera
                if let false=ok {
                    let ans=String::from("Unknown room, not able to join. Please [create]/[join room_name]/[print rooms] room.");
                    return ans;
                }
                else {
                    rooms.remove(room_name);
                    rooms.insert(room_name.to_string(), a);
                    //SendSuccessMessage();
                    //JoinRoom();
                    let ans=String::from("Succesfully joined: ");
                    return ans+room_name;
                }
            }
            "print" => {
                
                print_all_rooms(&rooms);
                return String::from("Printed all rooms.\n");
            }
            _ => {
                // unknown command
                let ans=String::from("Unknown command, please [create]/[join room_name] room.\n");
                return ans;
            }
        }
        
    }