pub fn run_tests(mut board: crate::board::Board) {
    let black_groups: Vec<crate::board::Loc> = vec![
        // Group 1
        crate::board::Loc::from_string("1, 1"),
        crate::board::Loc { row: 1, col: 2 },
        // Group 2
        crate::board::Loc { row: 4, col: 1 },
        crate::board::Loc { row: 5, col: 1 },
        // Group 3
        crate::board::Loc { row: 3, col: 3 },
        crate::board::Loc { row: 3, col: 4 },
        crate::board::Loc { row: 4, col: 3 },
        // Group 4
        crate::board::Loc { row: 4, col: 7 },
        crate::board::Loc { row: 5, col: 7 },
        crate::board::Loc { row: 6, col: 7 },
    ];
    let white_groups: Vec<crate::board::Loc> = vec![
        // Group 5
        crate::board::Loc { row: 2, col: 2 },
        crate::board::Loc { row: 3, col: 1 },
        crate::board::Loc { row: 3, col: 2 },
        crate::board::Loc { row: 4, col: 2 },
        // Group 6
        crate::board::Loc { row: 9, col: 1 },
        // Group 7
        crate::board::Loc { row: 6, col: 2 },
        crate::board::Loc { row: 6, col: 3 },
        crate::board::Loc { row: 7, col: 2 },
        crate::board::Loc { row: 7, col: 3 },
        crate::board::Loc { row: 8, col: 2 },
    ];

    for mv in black_groups {
        board.play(&crate::board::Move {
            player: crate::board::Player::Black,
            loc: mv,
        })
    }

    for mv in white_groups {
        board.play(&crate::board::Move {
            player: crate::board::Player::White,
            loc: mv,
        })
    }

    board.print_board();
    let group1_a = board.group_stones(crate::board::Loc { row: 1, col: 1 });
    let group1_b = board.group_stones(crate::board::Loc { row: 1, col: 2 });
    let group2_a = board.group_stones(crate::board::Loc { row: 4, col: 1 });
    let group2_b = board.group_stones(crate::board::Loc { row: 5, col: 1 });
    let group3_a = board.group_stones(crate::board::Loc { row: 3, col: 3 });
    let group3_b = board.group_stones(crate::board::Loc { row: 3, col: 4 });
    let group3_c = board.group_stones(crate::board::Loc { row: 4, col: 3 });
    let group4_a = board.group_stones(crate::board::Loc { row: 4, col: 7 });
    let group4_b = board.group_stones(crate::board::Loc { row: 5, col: 7 });
    let group4_c = board.group_stones(crate::board::Loc { row: 6, col: 7 });
    let group5_a = board.group_stones(crate::board::Loc { row: 2, col: 2 });
    let group5_b = board.group_stones(crate::board::Loc { row: 3, col: 1 });
    let group5_c = board.group_stones(crate::board::Loc { row: 3, col: 2 });
    let group5_d = board.group_stones(crate::board::Loc { row: 4, col: 2 });
    let group6 = board.group_stones(crate::board::Loc { row: 9, col: 1 });
    let group7_a = board.group_stones(crate::board::Loc { row: 6, col: 2 });
    let group7_b = board.group_stones(crate::board::Loc { row: 6, col: 3 });
    let group7_c = board.group_stones(crate::board::Loc { row: 7, col: 2 });
    let group7_d = board.group_stones(crate::board::Loc { row: 7, col: 3 });
    let group7_e = board.group_stones(crate::board::Loc { row: 8, col: 2 });
    assert!(
        group1_a
            == [
                crate::board::Loc { row: 1, col: 1 },
                crate::board::Loc { row: 1, col: 2 }
            ]
    );
    assert!(
        group1_b
            == [
                crate::board::Loc { row: 1, col: 1 },
                crate::board::Loc { row: 1, col: 2 }
            ]
    );
    assert!(
        group2_a
            == [
                crate::board::Loc { row: 4, col: 1 },
                crate::board::Loc { row: 5, col: 1 }
            ]
    );
    assert!(
        group2_b
            == [
                crate::board::Loc { row: 4, col: 1 },
                crate::board::Loc { row: 5, col: 1 }
            ]
    );
    assert!(
        group3_a
            == [
                crate::board::Loc { row: 3, col: 3 },
                crate::board::Loc { row: 3, col: 4 },
                crate::board::Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        group3_b
            == [
                crate::board::Loc { row: 3, col: 3 },
                crate::board::Loc { row: 3, col: 4 },
                crate::board::Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        group3_c
            == [
                crate::board::Loc { row: 3, col: 3 },
                crate::board::Loc { row: 3, col: 4 },
                crate::board::Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        group4_a
            == [
                crate::board::Loc { row: 4, col: 7 },
                crate::board::Loc { row: 5, col: 7 },
                crate::board::Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        group4_b
            == [
                crate::board::Loc { row: 4, col: 7 },
                crate::board::Loc { row: 5, col: 7 },
                crate::board::Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        group4_c
            == [
                crate::board::Loc { row: 4, col: 7 },
                crate::board::Loc { row: 5, col: 7 },
                crate::board::Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        group5_a
            == [
                crate::board::Loc { row: 2, col: 2 },
                crate::board::Loc { row: 3, col: 1 },
                crate::board::Loc { row: 3, col: 2 },
                crate::board::Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        group5_b
            == [
                crate::board::Loc { row: 2, col: 2 },
                crate::board::Loc { row: 3, col: 1 },
                crate::board::Loc { row: 3, col: 2 },
                crate::board::Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        group5_c
            == [
                crate::board::Loc { row: 2, col: 2 },
                crate::board::Loc { row: 3, col: 1 },
                crate::board::Loc { row: 3, col: 2 },
                crate::board::Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        group5_d
            == [
                crate::board::Loc { row: 2, col: 2 },
                crate::board::Loc { row: 3, col: 1 },
                crate::board::Loc { row: 3, col: 2 },
                crate::board::Loc { row: 4, col: 2 }
            ]
    );
    assert!(group6 == [crate::board::Loc { row: 9, col: 1 }]);
    assert!(
        group7_a
            == [
                crate::board::Loc { row: 6, col: 2 },
                crate::board::Loc { row: 6, col: 3 },
                crate::board::Loc { row: 7, col: 2 },
                crate::board::Loc { row: 7, col: 3 },
                crate::board::Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_b
            == [
                crate::board::Loc { row: 6, col: 2 },
                crate::board::Loc { row: 6, col: 3 },
                crate::board::Loc { row: 7, col: 2 },
                crate::board::Loc { row: 7, col: 3 },
                crate::board::Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_c
            == [
                crate::board::Loc { row: 6, col: 2 },
                crate::board::Loc { row: 6, col: 3 },
                crate::board::Loc { row: 7, col: 2 },
                crate::board::Loc { row: 7, col: 3 },
                crate::board::Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_d
            == [
                crate::board::Loc { row: 6, col: 2 },
                crate::board::Loc { row: 6, col: 3 },
                crate::board::Loc { row: 7, col: 2 },
                crate::board::Loc { row: 7, col: 3 },
                crate::board::Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        group7_e
            == [
                crate::board::Loc { row: 6, col: 2 },
                crate::board::Loc { row: 6, col: 3 },
                crate::board::Loc { row: 7, col: 2 },
                crate::board::Loc { row: 7, col: 3 },
                crate::board::Loc { row: 8, col: 2 }
            ]
    );

    assert!(board.count_liberties(crate::board::Loc { row: 1, col: 1 }) == 2);
    assert!(board.count_liberties(crate::board::Loc { row: 1, col: 2 }) == 2);
    assert!(board.count_liberties(crate::board::Loc { row: 4, col: 1 }) == 2);
    assert!(board.count_liberties(crate::board::Loc { row: 5, col: 1 }) == 2);
    assert!(board.count_liberties(crate::board::Loc { row: 3, col: 3 }) == 5);
    assert!(board.count_liberties(crate::board::Loc { row: 3, col: 4 }) == 5);
    assert!(board.count_liberties(crate::board::Loc { row: 4, col: 3 }) == 5);
    assert!(board.count_liberties(crate::board::Loc { row: 4, col: 7 }) == 8);
    assert!(board.count_liberties(crate::board::Loc { row: 5, col: 7 }) == 8);
    assert!(board.count_liberties(crate::board::Loc { row: 6, col: 7 }) == 8);
    assert!(board.count_liberties(crate::board::Loc { row: 2, col: 2 }) == 3);
    assert!(board.count_liberties(crate::board::Loc { row: 3, col: 1 }) == 3);
    assert!(board.count_liberties(crate::board::Loc { row: 3, col: 2 }) == 3);
    assert!(board.count_liberties(crate::board::Loc { row: 4, col: 2 }) == 3);
    assert!(board.count_liberties(crate::board::Loc { row: 9, col: 1 }) == 2);
    assert!(board.count_liberties(crate::board::Loc { row: 6, col: 2 }) == 9);
    assert!(board.count_liberties(crate::board::Loc { row: 6, col: 3 }) == 9);
    assert!(board.count_liberties(crate::board::Loc { row: 7, col: 2 }) == 9);
    assert!(board.count_liberties(crate::board::Loc { row: 7, col: 3 }) == 9);
    assert!(board.count_liberties(crate::board::Loc { row: 8, col: 2 }) == 9);

    board.remove_group(crate::board::Loc { row: 1, col: 1 });
    assert!(board.fields[1][1] == crate::board::Color::Empty);
    assert!(board.fields[1][2] == crate::board::Color::Empty);
    board.print_board();
    board.remove_group(crate::board::Loc { row: 5, col: 1 });
    assert!(board.fields[4][1] == crate::board::Color::Empty);
    assert!(board.fields[5][1] == crate::board::Color::Empty);
    board.print_board();
    board.remove_group(crate::board::Loc { row: 3, col: 4 });
    assert!(board.fields[3][3] == crate::board::Color::Empty);
    assert!(board.fields[3][4] == crate::board::Color::Empty);
    assert!(board.fields[4][3] == crate::board::Color::Empty);
    board.print_board();
    board.remove_group(crate::board::Loc { row: 6, col: 7 });
    assert!(board.fields[4][7] == crate::board::Color::Empty);
    assert!(board.fields[5][7] == crate::board::Color::Empty);
    assert!(board.fields[6][7] == crate::board::Color::Empty);
    board.print_board();
    board.remove_group(crate::board::Loc { row: 3, col: 2 });
    assert!(board.fields[2][2] == crate::board::Color::Empty);
    assert!(board.fields[3][1] == crate::board::Color::Empty);
    assert!(board.fields[3][2] == crate::board::Color::Empty);
    assert!(board.fields[4][2] == crate::board::Color::Empty);
    board.print_board();
    board.remove_group(crate::board::Loc { row: 9, col: 1 });
    assert!(board.fields[9][1] == crate::board::Color::Empty);
    board.print_board();
    board.remove_group(crate::board::Loc { row: 7, col: 3 });
    assert!(board.fields[6][2] == crate::board::Color::Empty);
    assert!(board.fields[6][3] == crate::board::Color::Empty);
    assert!(board.fields[7][2] == crate::board::Color::Empty);
    assert!(board.fields[7][3] == crate::board::Color::Empty);
    assert!(board.fields[8][2] == crate::board::Color::Empty);
    board.print_board();

    let black_groups: Vec<crate::board::Loc> = vec![
        // Group 1
        crate::board::Loc { row: 1, col: 1 },
        crate::board::Loc { row: 1, col: 2 },
        // Group 2
        crate::board::Loc { row: 4, col: 1 },
        crate::board::Loc { row: 5, col: 1 },
        // Group 3
        crate::board::Loc { row: 3, col: 3 },
        crate::board::Loc { row: 3, col: 4 },
        crate::board::Loc { row: 4, col: 3 },
        // Group 4
        crate::board::Loc { row: 4, col: 7 },
        crate::board::Loc { row: 5, col: 7 },
        crate::board::Loc { row: 6, col: 7 },
        crate::board::Loc { row: 7, col: 7 },
        // Group 5
        crate::board::Loc { row: 9, col: 9 },
    ];
    let white_groups: Vec<crate::board::Loc> = vec![
        // Takes group 1
        crate::board::Loc { row: 2, col: 1 },
        crate::board::Loc { row: 2, col: 2 },
        crate::board::Loc { row: 1, col: 3 },
        // Takes group 2
        crate::board::Loc { row: 3, col: 1 },
        crate::board::Loc { row: 2, col: 2 },
        crate::board::Loc { row: 5, col: 2 },
        crate::board::Loc { row: 6, col: 1 },
        // Takes group 3
        crate::board::Loc { row: 2, col: 3 },
        crate::board::Loc { row: 2, col: 4 },
        crate::board::Loc { row: 3, col: 2 },
        crate::board::Loc { row: 3, col: 5 },
        crate::board::Loc { row: 4, col: 2 },
        crate::board::Loc { row: 4, col: 4 },
        crate::board::Loc { row: 5, col: 3 },
        // Takes group 4
        crate::board::Loc { row: 3, col: 7 },
        crate::board::Loc { row: 4, col: 6 },
        crate::board::Loc { row: 4, col: 8 },
        crate::board::Loc { row: 5, col: 6 },
        crate::board::Loc { row: 5, col: 8 },
        crate::board::Loc { row: 6, col: 6 },
        crate::board::Loc { row: 6, col: 8 },
        crate::board::Loc { row: 7, col: 6 },
        crate::board::Loc { row: 7, col: 8 },
        crate::board::Loc { row: 8, col: 7 },
        // Takes group 5
        crate::board::Loc { row: 8, col: 9 },
        crate::board::Loc { row: 9, col: 8 },
    ];

    for mv in black_groups {
        board.play(&crate::board::Move {
            player: crate::board::Player::Black,
            loc: mv,
        });
    }

    board.print_board();

    for mv in white_groups {
        board.play(&crate::board::Move {
            player: crate::board::Player::White,
            loc: mv,
        });
        println!("After trying to remove a group after {:?} move", mv);
        board.print_board();
    }
}
