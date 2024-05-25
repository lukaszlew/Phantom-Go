use crate::board::{Board, Loc, Move, Player};

pub mod board;
pub mod board_test;

fn main() {
    let mut board = Board::new(7, 7, 0.0);
    let mut current_move = Move {
        player: Player::Black,
        loc: Loc { row: 0, col: 0 },
    };

    // Game loop
    loop {
        println!(
            "Turn: {:?}\nInput coordinates to play, 'u' to undo, 'p' to pass or 'q' to quit",
            current_move.player
        );
        let player_input = board::take_player_input();

        match player_input.as_str() {
            "q" => {
                println!("\nQuit game!\n");
                return;
            }
            "p" => {
                let previous_move = board.game_history.last();
                let previous_move_is_pass = match previous_move {
                    Some(previous_move) => previous_move.is_pass(),
                    None => false,
                };

                if previous_move_is_pass {
                    println!("\nGame ended!\n");
                    break;
                }

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

    // Removing dead stones loop
    loop {
        println!("\nRemove dead stones or input 'r' to calculate the result:\n");
        println!("{}", board.to_string());

        let player_input = board::take_player_input();
        match player_input.as_str() {
            "r" => break,
            _ => match Loc::from_string(&player_input) {
                None => {
                    println!("\nInvalid location :c\nInput one of the group's stone's location to remove it!");
                    continue;
                }
                Some(group_to_remove_loc) => board.remove_group(group_to_remove_loc),
            },
        }
    }
    board.print_result();
}
