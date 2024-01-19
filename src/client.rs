mod board;
use board::{Board, Cellprop};
use macroquad::prelude::*;
use std::f32::consts::PI;
use std::io::{self, Read, Write};
use std::str;
use std::thread;
const HEX_SIZE: f32 = 15.0;
const PADDING_TOP: f32 = 40.0;
const PADDING_LEFT: f32 = 40.0;

use std::net::TcpStream;
use std::sync::{Arc, Mutex};
/*
struct Button {
    rect: Rect,
    text: String,
    clicked: bool,
}

enum Gamestate {
    Gameboard,
    Waiting,
    Roomselect,
}
*/
fn calculate_hexagon_position(row: i32, col: i32, size: f32) -> (f32, f32) {
    let x = size * 3f32.sqrt() * (col as f32 + 0.5 * (row % 2) as f32) + PADDING_LEFT;
    let y = size * 1.5 * row as f32 + PADDING_TOP;
    (x, y)
}

fn draw_hexagon(x: f32, y: f32, size: f32, line_thickness: f32, filled: bool, color: Color) {
    let mut points = Vec::new();
    let rotation_offset = PI / 6.0;
    for i in 0..6 {
        let angle = rotation_offset + i as f32 * PI / 3.0;
        points.push(Vec2::new(x + size * angle.cos(), y + size * angle.sin()));
    }

    if filled {
        draw_poly(x, y, 6, size, rotation_offset, color);
    } else {
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];
            draw_line(p1.x, p1.y, p2.x, p2.y, line_thickness, color);
        }
    }
} /*
  fn button(button: &mut Button) -> bool {
      draw_rectangle(
          button.rect.x,
          button.rect.y,
          button.rect.w,
          button.rect.h,
          BLUE,
      );
      draw_text(
          &button.text,
          button.rect.x + 5.0,
          button.rect.y + button.rect.h / 2.0 + 5.0,
          30.0,
          WHITE,
      );

      let mouse_pos = mouse_position();
      if is_mouse_button_pressed(MouseButton::Left)
          && button.rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1))
      {
          button.clicked = true;
      }

      button.clicked
  }
  fn print_computer_or_waiting() -> String{
      clear_background(WHITE);
      let mut output=String::new();
      let mut wait_button = Button {
          rect: Rect::new(100.0, 150.0, 200.0, 50.0),
          text: "Wait for PL".to_string(),
          clicked: false,
      };
      let mut computer_button = Button {
          rect: Rect::new(100.0, 250.0, 200.0, 50.0),
          text: "Play WC".to_string(),
          clicked: false,
      };
      if button(&mut wait_button) {
          println!("wait_button clicked");
          output = String::from("W");
      }

      if button(&mut computer_button) {
          println!("computer_button clicked");
          output = String::from("C");
      }
      return output;
  }
  fn print_room_options(room_name:&str) -> String {
      clear_background(WHITE);
      let mut output = String::new();
      let mut create_room_button = Button {
          rect: Rect::new(100.0, 150.0, 200.0, 50.0),
          text: "Create Room".to_string(),
          clicked: false,
      };
      let mut join_room_button = Button {
          rect: Rect::new(100.0, 250.0, 200.0, 50.0),
          text: "Join Room".to_string(),
          clicked: false,
      };
      let mut room_name = String::new();

      if button(&mut create_room_button) {
          println!("Create Room clicked");
          output = String::from("create ");
      }

      if button(&mut join_room_button) {
          println!("Join Room clicked");
          output = String::from("join ");
      }
      draw_text(&format!("Room name: {}", room_name), 100.0, 350.0, 30.0, BLACK);
      output.push_str(room_name.as_str());
      return output;
  }*/

fn print_waiting() {
    clear_background(LIGHTGRAY); // Set a neutral background color
                                 // Display a waiting message
    draw_text(
        "Waiting for another player...",
        screen_width() / 2. - 150.,
        screen_height() / 2.,
        30.,
        BLACK,
    );
} /*
  fn print_game_loading() {
      clear_background(LIGHTGRAY); // Set a neutral background color
                                   // Display a waiting message
      draw_text(
          "Game loading...",
          screen_width() / 2. - 150.,
          screen_height() / 2.,
          30.,
          BLACK,
      );
  }*/

fn print_board(a: Board) {
    clear_background(WHITE);

    for row in 0..11 {
        for col in 0..11 {
            let (x, y) = calculate_hexagon_position(row, col, HEX_SIZE);
            if a.get_prop((row + 1, col + 1)) == Cellprop::Free {
                draw_hexagon(x, y, HEX_SIZE, 2.0, false, GRAY);
            } else if a.get_prop((row + 1, col + 1)) == Cellprop::Wall {
                draw_hexagon(x, y, HEX_SIZE, 2.0, true, BLACK);
            } else if a.get_prop((row + 1, col + 1)) == Cellprop::Mouse {
                draw_hexagon(x, y, HEX_SIZE, 2.0, true, RED);
            }
        }
    }
}

type SharedState = Arc<Mutex<Board>>; /*
                                      fn readfromserver(mut stream:&TcpStream)->String{

                                          let mut buffer = [0; 1024];
                                          let mut bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
                                          while bytes_read ==0 {
                                              bytes_read = stream.read(&mut buffer).expect("Failed to read from stream")
                                          }
                                          let inp = match str::from_utf8(&buffer[..bytes_read]) {
                                              Ok(v) => v.trim(),
                                              Err(_) => {
                                                  return String::from("Error");
                                              },
                                          };
                                          return String::from(inp);
                                      }*/
fn updbrd(state: &SharedState, a: String) {
    let mut brd = state.lock().unwrap();
    brd.plays();
    brd.translate_to_board(a);
}
fn isready(state: &SharedState) -> bool {
    let mut brd = state.lock().unwrap();
    brd.isgamestarted()
}
fn handle_servercomm(state: SharedState) {
    let mut stream = TcpStream::connect("127.0.0.1:7881").expect("Failed to connect to server");

    let mut nothing_to_read_next_time = false;
    loop {
        if !nothing_to_read_next_time {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read from stdin");

            stream
                .write_all(input.as_bytes())
                .expect("Failed to write to stream");
        }
        if nothing_to_read_next_time {
            // GB.print_for_TUI();
        }
        nothing_to_read_next_time = false;
        let mut buffer = [0; 1024];
        let mut bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from stream");
        while bytes_read == 0 {
            bytes_read = stream
                .read(&mut buffer)
                .expect("Failed to read from stream")
        }

        let input = match str::from_utf8(&buffer[..bytes_read]) {
            Ok(v) => v.trim(),
            Err(_) => {
                continue;
            }
        };
        if input.contains('{') {
            nothing_to_read_next_time = true;
            updbrd(&state, String::from(input));
        } else if input.contains("Game") {
            nothing_to_read_next_time = true;
        }
        println!(
            "Received: {}",
            str::from_utf8(&buffer[..bytes_read]).expect("Failed to read buffer")
        );
    }
}

#[macroquad::main("Trap the Mouse")]
async fn main() {
    let state = Arc::new(Mutex::new(Board::new()));
    //let mut shouldread_inp=false;

    let state_clone = state.clone();
    thread::spawn(move || {
        handle_servercomm(state_clone);
    });
    loop {
        let cpy = state.clone();
        if isready(&cpy) {
            let a = state.lock().unwrap().clone();
            print_board(a);
        } else {
            print_waiting();
        }
        next_frame().await;
    }
}

/*
        if pas==0 {
            let action = print_room_options(&input);

            stream.write_all(action.as_bytes()).expect("Failed to write to stream");
            server_response=readfromserver(&stream);
            if server_response.contains("Game") {
                pas+=2;
            }
            else if server_response.contains("join"){
                pas+=1;
            }
        }
        else if pas==1{

            let mut action=print_computer_or_waiting();
            action=String::from("C");
            stream.write_all(action.as_bytes()).expect("Failed to write to stream");
            pas+1;

        }
        else if pas==2 { //Game
            let mut buffer = [0; 1024];
            let mut bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
            while bytes_read ==0 {
                bytes_read = stream.read(&mut buffer).expect("Failed to read from stream")
            }
            let inp = match str::from_utf8(&buffer[..bytes_read]) {
                Ok(v) => v.trim(),
                Err(_) => {
                    continue;
                },
            };
            let mut GB:Board=Board::new();
            GB.translate_to_board(String::from(inp));
            print_board(GB);
            let mut action=String::from("1 1");
            stream.write_all(action.as_bytes()).expect("Failed to write to stream");
        }
*/
/*
fn print_selectorcreate(room_name: &str) -> String {
    clear_background(LIGHTGRAY);

    let (mouse_x, mouse_y) = mouse_position();

    // Define button and text input box bounds
    let create_btn_bounds = Rect::new(screen_width() / 2. - 150., screen_height() / 2. - 100., 300., 50.);
    let join_btn_bounds = Rect::new(screen_width() / 2. - 150., screen_height() / 2., 300., 50.);

    // Draw buttons and text input box
    draw_rectangle(create_btn_bounds.x, create_btn_bounds.y, create_btn_bounds.w, create_btn_bounds.h, BLUE);
    draw_text("Create Room", create_btn_bounds.x + 80., create_btn_bounds.y + 35., 30., WHITE);

    draw_rectangle(join_btn_bounds.x, join_btn_bounds.y, join_btn_bounds.w, join_btn_bounds.h, GREEN);
    draw_text("Join Room", join_btn_bounds.x + 90., join_btn_bounds.y + 35., 30., WHITE);

    draw_text(room_name, screen_width() / 2. - 140., screen_height() / 2. - 25., 30., BLACK);

    // Check for button clicks
    if is_mouse_button_pressed(MouseButton::Left) {
        if create_btn_bounds.contains(Vec2::new(mouse_x, mouse_y)) {
            "Create".to_string()
        } else if join_btn_bounds.contains(Vec2::new(mouse_x, mouse_y)) {
            "Join".to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}
 */
