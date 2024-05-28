use crate::board::{Board, Loc, Move, Player};

pub mod board;

fn main() {
    let mut board = Board::new(7, 7, 1.5);

    // TODO: Make this variable more local. Try to simplify code
    // BTW board has access to last move if you need it, including last player.

    let mut current_move = Move {
        player: Player::Black,
        loc: Loc { row: 0, col: 0 },
    };

    // Game loop
    while !board.last_two_moves_are_pass() {
        println!(
            "Turn: {:?}\nInput coordinates to play, 'u' to undo, 'p' to pass or 'q' to quit",
            current_move.player
        );
        let player_input = board::take_player_input();

        // TODO: This match is too long
        match player_input.as_str() {
            "q" => {
                println!("\nQuit game!\n");
                return;
            }
            "p" => {
                current_move = current_move.pass();
            }
            "gh" => {
                println!("\n\n{:?}\n\n", board.game_history);
                continue;
            }
            "u" => {
                board = board.undo();
                current_move.player = current_move.player.change();
                println!("{}", board.to_string());
                continue;
            }
            _ => match Loc::from_string(&player_input) {
                None => {
                    println!("\nInvalid move :c\nT R Y  A G A I N !\n");
                    continue;
                }
                Some(valid_loc_string) => current_move.loc = valid_loc_string,
            },
        }

        board.play(&current_move);
        current_move.player = current_move.player.change();
        println!("{}", board.to_string());
    }

    println!("{}", board.count_score().to_string());
}
