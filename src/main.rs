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

        if player_input == "q" {
            println!("\nQuit game!\n");
            return;
        } else if player_input == "p" {
            current_move.pass(
                &mut black_pass,
                &mut black_pass_counter,
                &mut white_pass,
                &mut white_pass_counter,
            );
            board.calculate_captures(black_pass_counter, white_pass_counter);
            board.print_board();
            if black_pass && white_pass {
                println!("Game ended!");
                break;
            }
            board.change_player(&mut current_move);
            continue;
        } else if player_input == "gh" {
            println!("\n\n{:?}\n\n", board.game_history);
        } else if player_input == "u" && board.game_history.len() != 0 {
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
        } else {
            black_pass = false;
            white_pass = false;
        }

        let coords = Loc::from_string(&player_input);
        let invalid_coord = Loc { row: 0, col: 0 };

        if coords == invalid_coord {
            println!("\nPut in coords in \"row_index, column_index format\" ");
            continue;
        }

        current_move.loc = coords;

        if !board.move_is_valid(&current_move) {
            println!("\nInvalid move :c\nT R Y  A G A I N !\n");
            continue;
        }

        board.play(&current_move);
        board.calculate_captures(black_pass_counter, white_pass_counter);
        board.change_player(&mut current_move);
        board.print_board();
    }

    println!("\nRemove dead stones or input 'r' to calculate the result:\n");
    // Removing dead stones loop
    loop {
        board.print_board();
        let player_input = board::take_player_input();

        if player_input == "r" {
            break;
        }
        board.remove_group(Loc::from_string(&player_input));
    }

    let captures = board.calculate_captures(black_pass_counter, white_pass_counter);
    let board_points = board.count_board_points();
    board.count_score(board_points, captures, board.komi);
}
