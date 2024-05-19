use std::io;

use crate::board::Player;

pub mod board;
pub mod board_test;

fn main() {
    board_test::run_tests();
    println!("\nAll tests P A S S E D !\n");
    println!("\nAfter all tests have passed... Your game may begin!\n\n");
    let mut board = board::Board::new(11, 11);
    let mut current_move = board::Move {
        player: board::Player::Black,
        loc: board::Loc { row: 0, col: 0 },
    };
    let komi: usize = 2;
    let mut black_pass: bool = false;
    let mut white_pass: bool = false;
    let mut black_pass_counter: usize = 0;
    let mut white_pass_counter: usize = 0;

    loop {
        println!("\nTurn: {:?}\n", current_move.player);
        let mut player_input: String = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read input");

        if player_input.trim() == "q" {
            println!("\nQuit game!\n");
            break;
        } else if player_input.trim() == "pass" {
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
                let captures = board.calculate_captures(black_pass_counter, white_pass_counter);
                let board_points = board.count_board_points();
                board.count_score(board_points, captures, komi);
                break;
            }
            board.change_player(&mut current_move);
            continue;
        } else if player_input.trim() == "gh" {
            println!("\n\n{:?}\n\n", board.game_history);
        } else if player_input.trim() == "u" && board.game_history.len() != 0 {
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

        let coords = board::Loc::from_string(&player_input);
        let invalid_coord = board::Loc { row: 0, col: 0 };

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
}
