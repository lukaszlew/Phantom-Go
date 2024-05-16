use rand::Rng;

use crate::board::Board;
use crate::board::Color;
use crate::board::Loc;
use crate::board::Move;
use crate::board::Player;

pub fn run_tests() {
    let mut board = Board::new(11, 11);

    let black_groups: Vec<Loc> = vec![
        // Group 1
        Loc::from_string("1, 1"),
        Loc { row: 1, col: 2 },
        // Group 2
        Loc { row: 4, col: 1 },
        Loc { row: 5, col: 1 },
        // Group 3
        Loc { row: 3, col: 3 },
        Loc { row: 3, col: 4 },
        Loc { row: 4, col: 3 },
        // Group 4
        Loc { row: 4, col: 7 },
        Loc { row: 5, col: 7 },
        Loc { row: 6, col: 7 },
    ];
    let white_groups: Vec<Loc> = vec![
        // Group 5
        Loc { row: 2, col: 2 },
        Loc { row: 3, col: 1 },
        Loc { row: 3, col: 2 },
        Loc { row: 4, col: 2 },
        // Group 6
        Loc { row: 9, col: 1 },
        // Group 7
        Loc { row: 6, col: 2 },
        Loc { row: 6, col: 3 },
        Loc { row: 7, col: 2 },
        Loc { row: 7, col: 3 },
        Loc { row: 8, col: 2 },
    ];

    for mv in black_groups {
        board.play_if_move_is_valid(&Move {
            player: Player::Black,
            loc: mv,
        })
    }

    for mv in white_groups {
        board.play_if_move_is_valid(&Move {
            player: Player::White,
            loc: mv,
        })
    }

    board.print_board();
    let group1_a = board.group_stones(Loc { row: 1, col: 1 });
    let group1_b = board.group_stones(Loc { row: 1, col: 2 });
    let group2_a = board.group_stones(Loc { row: 4, col: 1 });
    let group2_b = board.group_stones(Loc { row: 5, col: 1 });
    let group3_a = board.group_stones(Loc { row: 3, col: 3 });
    let group3_b = board.group_stones(Loc { row: 3, col: 4 });
    let group3_c = board.group_stones(Loc { row: 4, col: 3 });
    let group4_a = board.group_stones(Loc { row: 4, col: 7 });
    let group4_b = board.group_stones(Loc { row: 5, col: 7 });
    let group4_c = board.group_stones(Loc { row: 6, col: 7 });
    let group5_a = board.group_stones(Loc { row: 2, col: 2 });
    let group5_b = board.group_stones(Loc { row: 3, col: 1 });
    let group5_c = board.group_stones(Loc { row: 3, col: 2 });
    let group5_d = board.group_stones(Loc { row: 4, col: 2 });
    let group6 = board.group_stones(Loc { row: 9, col: 1 });
    let group7_a = board.group_stones(Loc { row: 6, col: 2 });
    let group7_b = board.group_stones(Loc { row: 6, col: 3 });
    let group7_c = board.group_stones(Loc { row: 7, col: 2 });
    let group7_d = board.group_stones(Loc { row: 7, col: 3 });
    let group7_e = board.group_stones(Loc { row: 8, col: 2 });
    assert!(group1_a == [Loc { row: 1, col: 1 }, Loc { row: 1, col: 2 }]);
    assert!(group1_b == [Loc { row: 1, col: 1 }, Loc { row: 1, col: 2 }]);
    assert!(group2_a == [Loc { row: 4, col: 1 }, Loc { row: 5, col: 1 }]);
    assert!(group2_b == [Loc { row: 4, col: 1 }, Loc { row: 5, col: 1 }]);
    assert!(
        group3_a
            == [
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        group3_b
            == [
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        group3_c
            == [
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        group4_a
            == [
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        group4_b
            == [
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        group4_c
            == [
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        group5_a
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        group5_b
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        group5_c
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        group5_d
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(group6 == [Loc { row: 9, col: 1 }]);
    assert!(
        group7_a
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_b
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_c
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_d
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_e
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );

    assert!(board.count_liberties(Loc { row: 1, col: 1 }) == 2);
    assert!(board.count_liberties(Loc { row: 1, col: 2 }) == 2);
    assert!(board.count_liberties(Loc { row: 4, col: 1 }) == 2);
    assert!(board.count_liberties(Loc { row: 5, col: 1 }) == 2);
    assert!(board.count_liberties(Loc { row: 3, col: 3 }) == 5);
    assert!(board.count_liberties(Loc { row: 3, col: 4 }) == 5);
    assert!(board.count_liberties(Loc { row: 4, col: 3 }) == 5);
    assert!(board.count_liberties(Loc { row: 4, col: 7 }) == 8);
    assert!(board.count_liberties(Loc { row: 5, col: 7 }) == 8);
    assert!(board.count_liberties(Loc { row: 6, col: 7 }) == 8);
    assert!(board.count_liberties(Loc { row: 2, col: 2 }) == 3);
    assert!(board.count_liberties(Loc { row: 3, col: 1 }) == 3);
    assert!(board.count_liberties(Loc { row: 3, col: 2 }) == 3);
    assert!(board.count_liberties(Loc { row: 4, col: 2 }) == 3);
    assert!(board.count_liberties(Loc { row: 9, col: 1 }) == 2);
    assert!(board.count_liberties(Loc { row: 6, col: 2 }) == 9);
    assert!(board.count_liberties(Loc { row: 6, col: 3 }) == 9);
    assert!(board.count_liberties(Loc { row: 7, col: 2 }) == 9);
    assert!(board.count_liberties(Loc { row: 7, col: 3 }) == 9);
    assert!(board.count_liberties(Loc { row: 8, col: 2 }) == 9);

    board.remove_group(Loc { row: 1, col: 1 });
    assert!(board.fields[1][1] == Color::Empty);
    assert!(board.fields[1][2] == Color::Empty);
    board.print_board();
    board.remove_group(Loc { row: 5, col: 1 });
    assert!(board.fields[4][1] == Color::Empty);
    assert!(board.fields[5][1] == Color::Empty);
    board.print_board();
    board.remove_group(Loc { row: 3, col: 4 });
    assert!(board.fields[3][3] == Color::Empty);
    assert!(board.fields[3][4] == Color::Empty);
    assert!(board.fields[4][3] == Color::Empty);
    board.print_board();
    board.remove_group(Loc { row: 6, col: 7 });
    assert!(board.fields[4][7] == Color::Empty);
    assert!(board.fields[5][7] == Color::Empty);
    assert!(board.fields[6][7] == Color::Empty);
    board.print_board();
    board.remove_group(Loc { row: 3, col: 2 });
    assert!(board.fields[2][2] == Color::Empty);
    assert!(board.fields[3][1] == Color::Empty);
    assert!(board.fields[3][2] == Color::Empty);
    assert!(board.fields[4][2] == Color::Empty);
    board.print_board();
    board.remove_group(Loc { row: 9, col: 1 });
    assert!(board.fields[9][1] == Color::Empty);
    board.print_board();
    board.remove_group(Loc { row: 7, col: 3 });
    assert!(board.fields[6][2] == Color::Empty);
    assert!(board.fields[6][3] == Color::Empty);
    assert!(board.fields[7][2] == Color::Empty);
    assert!(board.fields[7][3] == Color::Empty);
    assert!(board.fields[8][2] == Color::Empty);
    board.print_board();

    let black_groups: Vec<Loc> = vec![
        // Group 1
        Loc { row: 1, col: 1 },
        Loc { row: 1, col: 2 },
        // Group 2
        Loc { row: 4, col: 1 },
        Loc { row: 5, col: 1 },
        // Group 3
        Loc { row: 3, col: 3 },
        Loc { row: 3, col: 4 },
        Loc { row: 4, col: 3 },
        // Group 4
        Loc { row: 4, col: 7 },
        Loc { row: 5, col: 7 },
        Loc { row: 6, col: 7 },
        Loc { row: 7, col: 7 },
        // Group 5
        Loc { row: 9, col: 9 },
    ];
    let white_groups: Vec<Loc> = vec![
        // Takes group 1
        Loc { row: 2, col: 1 },
        Loc { row: 2, col: 2 },
        Loc { row: 1, col: 3 },
        // Takes group 2
        Loc { row: 3, col: 1 },
        Loc { row: 2, col: 2 },
        Loc { row: 5, col: 2 },
        Loc { row: 6, col: 1 },
        // Takes group 3
        Loc { row: 2, col: 3 },
        Loc { row: 2, col: 4 },
        Loc { row: 3, col: 2 },
        Loc { row: 3, col: 5 },
        Loc { row: 4, col: 2 },
        Loc { row: 4, col: 4 },
        Loc { row: 5, col: 3 },
        // Takes group 4
        Loc { row: 3, col: 7 },
        Loc { row: 4, col: 6 },
        Loc { row: 4, col: 8 },
        Loc { row: 5, col: 6 },
        Loc { row: 5, col: 8 },
        Loc { row: 6, col: 6 },
        Loc { row: 6, col: 8 },
        Loc { row: 7, col: 6 },
        Loc { row: 7, col: 8 },
        Loc { row: 8, col: 7 },
        // Takes group 5
        Loc { row: 8, col: 9 },
        Loc { row: 9, col: 8 },
    ];

    for mv in black_groups {
        board.play_if_move_is_valid(&Move {
            player: Player::Black,
            loc: mv,
        });
    }

    board.print_board();

    for mv in white_groups {
        board.play_if_move_is_valid(&Move {
            player: Player::White,
            loc: mv,
        });
        println!("After trying to remove a group after {:?} move", mv);
        board.print_board();
    }

    let mut rng = rand::thread_rng();
    let mut current_move = Move {
        player: Player::White,
        loc: Loc { row: 0, col: 0 },
    };

    let mut board = Board::new(7, 7);
    let mut moves_left = 10;

    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = Loc { row, col };
        current_move.loc = current_move_coords;

        if board.move_is_valid(&current_move) {
            board.play_if_move_is_valid(&current_move);
            board.change_player(&mut current_move);
            board.print_board();
            println!();
            moves_left -= 1;
        }
    }

    println!("\nF I N A L  B O A R D:\n\n");
    board.print_board();
    board = board.undo();
    println!("\n1st undo:\n");
    board.print_board();
    board = board.undo();
    println!("\n2nd undo:\n");
    board.print_board();
    board = board.undo();
    println!("\n3rd undo:\n");
    board.print_board();
    board = board.undo();
    println!("\n4th undo:\n");
    board.print_board();
    board = board.undo();
    println!("\n5th undo:\n");
    board.print_board();
    board = board.undo();
    println!("\n6th undo:\n");
    board.print_board();

    println!("\n\nContinuing after UNDOS!\n\n");
    moves_left = 6;

    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = Loc { row, col };
        current_move.loc = current_move_coords;

        if board.move_is_valid(&current_move) {
            board.play_if_move_is_valid(&current_move);
            board.change_player(&mut current_move);
            board.print_board();
            println!();
            moves_left -= 1;
        }
    }

    println!("\nF I N A L  B O A R D:\n\n");
    board.print_board();
    println!();

    println!("\nDifficult test for undo:\n\n(1,1) and (2,1) stones have been captured before\n\n");
    let mut board = Board::new(7, 5);
    let moves = [
        Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        },
        Move {
            player: Player::White,
            loc: Loc { row: 1, col: 2 },
        },
        Move {
            player: Player::Black,
            loc: Loc { row: 2, col: 1 },
        },
        Move {
            player: Player::White,
            loc: Loc { row: 2, col: 2 },
        },
        Move {
            player: Player::Black,
            loc: Loc { row: 3, col: 2 },
        },
        Move {
            player: Player::White,
            loc: Loc { row: 3, col: 1 },
        },
        Move {
            player: Player::Black,
            loc: Loc { row: 4, col: 1 },
        },
        Move {
            player: Player::White,
            loc: Loc { row: 4, col: 2 },
        },
        Move {
            player: Player::Black,
            loc: Loc { row: 2, col: 1 },
        },
    ];
    for mv in moves {
        board.play_if_move_is_valid(&mv);
    }
    board.print_board();
    println!("\nAfter this undo, (2,1) stone should disappear and (3,1) stone appear.");
    println!("Other stones should not appear!\n");
    board = board.undo();
    board.print_board();
    assert!(board.fields[1][1] == Color::Empty);
    assert!(board.fields[2][1] == Color::Empty);
    assert!(board.fields[3][1] == Color::White);

    println!("\n\nTest for KO:\n");
    let mut board = Board::new(6, 5);
    let moves = [
        Move {
            player: Player::Black,
            loc: Loc { row: 3, col: 1 },
        },
        Move {
            player: Player::White,
            loc: Loc { row: 2, col: 1 },
        },
        Move {
            player: Player::Black,
            loc: Loc { row: 2, col: 2 },
        },
        Move {
            player: Player::White,
            loc: Loc { row: 1, col: 2 },
        },
        Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        },
    ];
    for mv in moves {
        board.play_if_move_is_valid(&mv);
        board.print_board();
        println!();
    }
    println!("Black just captured the stone at (2,1) with the move at (1,1):\n");
    board.print_board();
    println!("\n\nWhite tries to play, but shouldn't be able to recapture:\n\n");
    board.play_if_move_is_valid(&Move {
        player: Player::White,
        loc: Loc { row: 2, col: 1 },
    });
    board.print_board();
    assert!(board.fields[2][1] == Color::Empty);
    assert!(board.fields[1][1] == Color::Black);

    println!("\n\nBlack and White have to play elsewhere:\n\n");
    board.play_if_move_is_valid(&Move {
        player: Player::White,
        loc: Loc { row: 4, col: 3 },
    });
    board.play_if_move_is_valid(&Move {
        player: Player::Black,
        loc: Loc { row: 3, col: 3 },
    });
    board.print_board();
    println!("\n\nNow White can capture the KO:\n\n");
    board.play_if_move_is_valid(&Move {
        player: Player::White,
        loc: Loc { row: 2, col: 1 },
    });
    board.print_board();
    println!("\n\nBlack tries to capture, but can't:\n\n");
    board.play_if_move_is_valid(&Move {
        player: Player::Black,
        loc: Loc { row: 1, col: 1 },
    });
    board.print_board();
    assert!(board.fields[1][1] == Color::Empty);
    assert!(board.fields[2][1] == Color::White);
    println!("\n\nSo They have to play elsewhere:\n\n");
    board.play_if_move_is_valid(&Move {
        player: Player::Black,
        loc: Loc { row: 2, col: 3 },
    });
    board.play_if_move_is_valid(&Move {
        player: Player::White,
        loc: Loc { row: 4, col: 2 },
    });
    board.print_board();
    println!("\n\nFor Black to be able to recapture:\n\n");
    board.play_if_move_is_valid(&Move {
        player: Player::Black,
        loc: Loc { row: 1, col: 1 },
    });
    board.print_board();
    assert!(board.fields[2][1] == Color::Empty);
    assert!(board.fields[1][1] == Color::Black);

    println!("\n\nTests for passing:\n");
    let mut black_pass = false;
    let mut white_pass = false;
    let mut mv = Move {
        player: Player::Black,
        loc: Loc::from_string("3,3"),
    };
    assert!(black_pass == false);
    assert!(white_pass == false);
    mv.pass(&mut black_pass, &mut white_pass);
    println!("{:?} {:?}", black_pass, white_pass);
    assert!(black_pass == true);
    assert!(white_pass == false);
    mv.player = Player::White;
    mv.pass(&mut black_pass, &mut white_pass);
    assert!(black_pass == true);
    assert!(white_pass == true);
}
