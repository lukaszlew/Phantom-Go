use crate::board::{Board, Loc, Move, Player};

pub mod board;
pub mod board_test;

fn main() {
    let mut board = Board::new(7, 7, 2);
    let mut current_move = Move {
        player: Player::Black,
        loc: Loc { row: 0, col: 0 },
    };
    let mut black_pass: bool = false;
    let mut white_pass: bool = false;
    let mut black_pass_counter: usize = 0;
    let mut white_pass_counter: usize = 0;

    // Game loop
    loop {
        println!(
            "Turn: {:?}\nInput coordinates to play, 'u' to undo, 'p' to pass or 'q' to quit",
            current_move.player
        );
        let player_input = board::take_player_input();
        let mut loc: Loc = Loc { row: 0, col: 0 };

        match player_input.as_str() {
            "q" => {
                println!("\nQuit game!\n");
                return;
            }
            "p" => {
                current_move.pass(
                    &mut black_pass,
                    &mut black_pass_counter,
                    &mut white_pass,
                    &mut white_pass_counter,
                );
                board.calculate_captures(black_pass_counter, white_pass_counter);
                board.print_board();
                if black_pass && white_pass {
                    println!("\nGame ended!\n1,2");
                    break;
                }
                board.change_player(&mut current_move);
                continue;
            }
            "gh" => println!("\n\n{:?}\n\n", board.game_history),
            "u" => {
                if black_pass || white_pass {
                    match current_move.player {
                        Player::Black => white_pass_counter -= 1,
                        Player::White => black_pass_counter -= 1,
                    }
                    board.change_player(&mut current_move);
                    board.print_board();
                    continue;
                }
                board = board.undo();
                board.change_player(&mut current_move);
                board.calculate_captures(black_pass_counter, white_pass_counter);
                board.print_board();
                continue;
            }
            _ => match Loc::from_string(&player_input) {
                None => {
                    println!("\nInvalid move :c\nT R Y  A G A I N !\n");
                    continue;
                }
                Some(valid_loc_string) => loc = valid_loc_string,
            },
        }

        black_pass = false;
        white_pass = false;

        current_move.loc = loc;

        if !board.move_is_valid(&current_move) {
            println!("\nInvalid move :c\nT R Y  A G A I N !\n");
            continue;
        }

        board.play(&current_move);
        board.calculate_captures(black_pass_counter, white_pass_counter);
        board.change_player(&mut current_move);
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

    let captures = board.calculate_captures(black_pass_counter, white_pass_counter);
    let board_points = board.count_board_points();
    board.count_score(board_points, captures, board.komi);
}
