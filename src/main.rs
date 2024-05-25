use crate::board::{Board, Loc, Move, Player};

pub mod board;
pub mod board_test;

fn main() {
    let mut board = Board::new(7, 7, 2);
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
                    Some(previous_move) => previous_move.loc.row == 99,
                    None => false,
                };

                if previous_move_is_pass {
                    println!("\nGame ended!\n");
                    break;
                }

                let pass: Move = Move {
                    player: current_move.player.clone(),
                    loc: Loc { row: 99, col: 99 },
                };

                board.game_history.push(pass);

                board.pass(&current_move);
                board.print_board();

                current_move.player = current_move.player.change();
                continue;
            }
            "gh" => {
                println!("\n\n{:?}\n\n", board.game_history);
                continue;
            }
            "u" => {
                board = board.undo();
                board.print_board();
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

        if !board.move_is_valid(&current_move) {
            println!("\nInvalid move :c\nT R Y  A G A I N !\n");
            continue;
        }

        board.play(&current_move);
        current_move.player = current_move.player.change();
        board.print_board();
    }

    // Removing dead stones loop
    loop {
        println!("\nRemove dead stones or input 'r' to calculate the result:\n");
        board.print_board();

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
}
