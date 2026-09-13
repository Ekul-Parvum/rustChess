#![allow(non_snake_case)]
#![allow(clippy::manual_find)]

use std::io::{self, Write};


struct Player {
    color: String
}

impl Player {
    fn new(color: String) -> Player {
        Player {
            color
        }
    }

    fn getInput(&self) -> String {
        print!("{}: ", self.color);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        input
    }
}

struct Piece {
    pos_x: i32,
    pos_y: i32,
    asciiArtTop: String,
    isWhite: bool
}

impl Piece {
    fn new(pos_x: i32, pos_y: i32, isWhite: bool) -> Piece {
        Piece {
            pos_x,
            pos_y,
            asciiArtTop:    "GP".to_string(),
            isWhite
        }
    }
}

struct Board {
    player_w: Player,
    player_b: Player,
    pieces: Vec<Piece>,
    whitesTurn: bool
}

impl Board {
    fn new() -> Board {
        Board {
            player_w: Player::new("white".to_string()),
            player_b: Player::new("black".to_string()),
            pieces: vec![
                Piece::new(1, 1, true)
            ],
            whitesTurn: true
        }
    }

    fn getPlayerInput(&mut self) -> String {
        // Get Player Input:
        let playerInput: String = if self.whitesTurn {
            self.player_w.getInput()
        } else {
            self.player_b.getInput()
        };

        // Return
        playerInput
    }

    fn checkPosForPiece(&self, pos_x: i32, pos_y: i32) -> Option<&Piece> {
        for piece in &self.pieces {
            if piece.pos_x == pos_x && piece.pos_y == pos_y {
                return Some(piece);
            }
        }

        None
    }

    fn makeBoard(&self) -> String {
        let mut boardString: String = String::new();

        // Loop through each row and column:
        for row in 1..17 {
            if row % 2 == 0 {
                boardString.push_str("  |")
            } else {
                boardString.push_str(&format!("{} |", ((row + 1)/2)));
            }

            for col in 1..9 {
                // Check if there is a piece on this square:
                let square_x = (row + 1) / 2;
                let square_y = col;
                let potentialPiece: Option<&Piece> = self.checkPosForPiece(square_x, square_y);
                
                // Get what color this square is:
                let isXXX_Square: bool = (square_x + col) % 2 == 0;

                // Check if potentialPiece actualy found anything:
                if let Some(piece) = potentialPiece {
                    // Now we know that a piece is in this square:
                    // Now, we need to know if we are in the top or bottom half of the square:

                    // If this is the top row
                    if row % 2 != 0 {
                        // Then first we print what color the piece is:
                        if piece.isWhite {
                            boardString.push('W');
                        } else {
                            boardString.push('B');
                        }

                        boardString.push_str(piece.asciiArtTop.as_str());
                    } else {
                        // Print an empty bottom of the square:
                        if isXXX_Square {
                            boardString.push_str("XXX");
                        } else {
                            boardString.push_str("   ");
                        }
                    }
                } else {
                    // Print an empty checker square:
                    if isXXX_Square {
                        boardString.push_str("XXX");
                    } else {
                        boardString.push_str("   ");
                    }
                }
                boardString.push('|')
            }
            boardString.push('\n')
        }

        boardString
    }
}

fn main() {
    println!("Starting...");

    let mut board: Board = Board::new();
    let mut quit: bool = false;

    while !quit {
        // Clear the screen:
        print!("\x1B[2J\x1B[3J\x1B[1;1H");
        io::stdout().flush().unwrap();

        // Print board:
        let boardString: String = board.makeBoard();
        println!("{}", boardString);

        // Get playerInput input:
        let playerInput: String = board.getPlayerInput();
        print!("{}", playerInput);
        io::stdout().flush().unwrap();
        
        // Check if a player quit:
        if playerInput.trim() == "q" {
            quit = true;
        }

        // Switch who's turn it is:
        board.whitesTurn = !board.whitesTurn;
    }
}
