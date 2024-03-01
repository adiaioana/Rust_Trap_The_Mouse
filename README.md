# Trap the Mouse Game

A "Trap the Mouse" game implemented using a server that manages game logic and a client that displays the GUI. This project was developed as part of the "Rust Programming" course during the first semester of the second year of studying computer science at FII, Iasi.

## Description

The application allows clients to join or create a room to play the game either with a computer user or another user connected to the room. One of the most notable features is the architecture of the client's program. Utilizing multithreading, the client operates two threads:

1. **Game Logic Communication:** This thread handles communication with the server regarding game logic. It sends the updated game board to the server and receives the updated game board from the server. Additionally, it receives input data from the application's console.

2. **Graphical User Interface:** This thread is responsible for displaying the graphical user interface of the game board. The game board is a resource shared by both threads, managed using a mutex.

## Skills Engaged

- **Rust:** The main programming language used for development.
- **Data Structures:** Employed for efficient storage and manipulation of game data.
- **Multithreading:** Utilized for concurrent execution of tasks, enhancing performance.
- **Object-oriented Programming:** Possibly utilized for organizing code and implementing certain components.
