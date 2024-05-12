use std::io;
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

    loop {
        let mut player_input: String = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read input");

        if player_input.trim() == "undo" {
            println!("\nInside the if statement!!! \\o/ ");
            board = board.undo();
            board.change_player(&mut current_move);
            println!("{:?}", board.game_history);
            board.print_board();
            continue;
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

        println!("{:?}", current_move.loc);
        board.play(&current_move);
        board.change_player(&mut current_move);
        board.print_board();
    }
}
