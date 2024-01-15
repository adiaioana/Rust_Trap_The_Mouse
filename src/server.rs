use std::collections::HashMap;
use std::hash::Hash;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct Room {
    RoomNb: i32,
    RoomName: String,
    Player1: i32,
    Player2: i32,
    GameState: i32,
}
impl Room{
    pub fn new() -> Self {
        Room{RoomNb:0, RoomName:String::from(" "),Player1:0,Player2:0,GameState:0}
    }
}

type SharedState = Arc<Mutex<HashMap<String, Room>>>;
///
fn room_handling_for_client(mut buffer:[u8;1024], mut state: SharedState, idclient:i32) -> String{ 
    let cpy_buff=str::from_utf8(&buffer).unwrap();
    let recv_mess=parse_message(&cpy_buff);
    let arg1:&str=&recv_mess.0.as_str();
    let arg2:&str=&recv_mess.1.as_str();
    let mess=process_room_request(arg1, arg2, &state, idclient);
    mess
}
fn GetClientOption(stream: &mut TcpStream, state: &SharedState) -> char {
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
            let message = "Will be waiting for another player...\n";
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


fn handle_client(mut stream: TcpStream, mut state: SharedState, idclient:i32) {
    ///Step 1: Join a room
    let mut buffer = [0; 1024]; let mut whichroom=String::from("");
    while let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream") >0 {
        let mut mess=room_handling_for_client(buffer, state.clone(), idclient);
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
    ///Wait/Play(w Player/Computer)
    let gs=gamestate(&whichroom, &state, idclient);
    if gs==0 {
        //Option to wait or to play with computer
        let client_option= GetClientOption(&mut stream,&state);
        if client_option=='W' {
            let mut Begin=0;
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => Begin= n.as_secs(),
                Err(_) => panic!("SystemTime not working!"),
            }

            while let gs=gamestate(&whichroom, &state, idclient)!=1 {
                let mut TimeNow:u64=0;
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => Begin= n.as_secs(),
                    Err(_) => panic!("SystemTime not working!"),
                }
                if TimeNow-Begin>300 {
                    let mess="Waited too long. Ending application\n";
                    stream.write_all(mess.as_bytes()).expect("Failed to write to stream");
                    return ;
                }
            }
        }
        else {
            playing_with_computer=1;
        }
    }

    //Now, it's game time :0
    let mess="Game time\n";
    stream.write_all(mess.as_bytes()).expect("Failed to write to stream");
    
}

fn main() {
    let mut no_clients=0;
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    let state = Arc::new(Mutex::new(HashMap::new()));

    for mut stream in listener.incoming() {
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
        fn gamestate(room_name: &str, state: &SharedState, idc: i32)->i32 {
            let mut rooms = state.lock().unwrap();
            for it in rooms.iter() {
                if it.0.contains(room_name) {
                    return it.1.GameState;
                }
            }
            return -1;
        }
        fn parse_message(msg: &str) -> (String, String) { ///
            let s:String=msg.to_string();
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
                println!("Room with name {} has id {} with player1={},player2={} and gamestate={}", 
                roomy.0,roomy.1.RoomNb, roomy.1.Player1,roomy.1.Player2,roomy.1.GameState);
            }
        }
        fn process_room_request(command: &str, room_name: &str, state: &SharedState, idc: i32) -> String{
        // proceseaza requesturile legate de camere
        let mut rooms = state.lock().unwrap();
        match command {
            "create" => {
                // Create a new room and add it to the global state
                let lg=rooms.len();
                let mut A: Room=Room{RoomNb:lg as i32, RoomName:String::from(room_name),Player1:0,Player2:0,GameState:0};
                rooms.insert(room_name.to_string(),A);
                let ans=String::from("Succesfully created room: ");
                return ans+room_name;
            }
            "join" => {
                let mut ok:bool=false;
                let mut A:Room=Room::new();
                for it in rooms.iter() {
                    if it.0.contains(room_name) {
                        ok=true;
                        A=it.1.clone();
                        if A.Player1==0 {
                            A.Player1=idc;
                        }
                        else{
                            A.Player2=idc;
                            A.GameState=1;
                        }
                        break;
                    }
                }
                // verifica daca exista camera
                if let false=ok {
                    let ans=String::from("Unknown room, not able to join. Please [create]/[join roomname]/[print rooms] room.");
                    return ans;
                }
                else {
                    rooms.remove(room_name);
                    rooms.insert(room_name.to_string(), A);
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
                let ans=String::from("Unknown command, please [create]/[join roomname] room.\n");
                return ans;
            }
        }
        
    }