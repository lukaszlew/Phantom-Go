use crate::board::{Board, Loc, Move};

pub mod board;

fn main() {
    let mut board = Board::new(7, 7, 1.5);

    // Game loop
    while !board.last_two_moves_are_pass() {
        println!(
            "Turn: {:?}\nInput coordinates to play, 'u' to undo, 'p' to pass or 'q' to quit",
            board.get_current_player()
        );
        let player_input = board::take_player_input();

        match player_input.as_str() {
            "q" => {
                println!("\nQuit game!\n");
                break
            }
            "p" => board.play(&Move {
                player: board.get_current_player(),
                loc: Loc::pass(),
            }),
            "gh" => {
                println!("\n\n{:?}\n\n", board.get_game_history());
            }
            "u" => {
                board = board.undo();
            }
            _ => match Loc::from_string(&player_input) {
                None => {
                    println!("\nInvalid move :c\nT R Y  A G A I N !\n");
                }
                Some(loc) => {
                    let player = board.get_current_player()
                    board = board.play(Move { player, loc });
                }
            },
        }
        println!("{}", board.to_string());
    }

    println!("{}", board.count_score().to_string());
}
