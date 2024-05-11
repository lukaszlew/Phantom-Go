use rand::Rng;
use std::io;
pub mod board;
pub mod board_test;

fn main() {
    let board = board::Board::new(11, 11);
    let mut game_record: Vec<board::Move> = vec![];
    println!();
    board_test::run_tests(board);
    println!("\nAll tests P A S S E D !\n");

    let mut rng = rand::thread_rng();
    let mut current_move = board::Move {
        player: board::Player::Black,
        loc: board::Loc { row: 0, col: 0 },
    };

    let mut board = board::Board::new(7, 7);
    let mut moves_left = 10;

    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = board::Loc { row, col };
        current_move.loc = current_move_coords;

        if board.move_is_valid(&current_move) {
            game_record.push(current_move.clone());
            board.play(&current_move);
            board.change_player(&mut current_move);
            board.print_board();
            println!();
            moves_left -= 1;
        }
    }

    println!("\nF I N A L  B O A R D:\n\n");
    board.print_board();
    let board = board.undo(&mut game_record);
    println!("\n1st undo:\n");
    board.print_board();
    let board = board.undo(&mut game_record);
    println!("\n2nd undo:\n");
    board.print_board();
    let board = board.undo(&mut game_record);
    println!("\n3rd undo:\n");
    board.print_board();
    let board = board.undo(&mut game_record);
    println!("\n4th undo:\n");
    board.print_board();
    let board = board.undo(&mut game_record);
    println!("\n5th undo:\n");
    board.print_board();
    let mut board = board.undo(&mut game_record);
    println!("\n6th undo:\n");
    board.print_board();

    println!("\n\nContinuing after UNDOS!\n\n");
    moves_left = 6;

    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = board::Loc { row, col };
        current_move.loc = current_move_coords;

        if board.move_is_valid(&current_move) {
            game_record.push(current_move.clone());
            board.play(&current_move);
            board.change_player(&mut current_move);
            board.print_board();
            println!();
            moves_left -= 1;
        }
    }

    println!("\nF I N A L  B O A R D:\n\n");
    board.print_board();

    println!("\nAfter all tests have passed... Your game may begin!\n\n");
    let mut board = board::Board::new(11, 11);
    let mut current_move = board::Move {
        player: board::Player::Black,
        loc: board::Loc { row: 0, col: 0 },
    };

    loop {
        let mut player_input: String = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read input");
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

        println!("{:?}", current_move.loc);
        board.play(&current_move);
        board.change_player(&mut current_move);
        board.print_board();
    }
}
