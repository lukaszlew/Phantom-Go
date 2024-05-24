use crate::board::Loc;
use crate::board::Move;
use crate::board::Player;

pub fn run_tests() {
    // TODO: tests for passing will be different, because of the possible architecture change
    println!("\n\nTests for passing:\n");
    let mut black_pass = false;
    let mut white_pass = false;
    let mut mv = Move {
        player: Player::Black,
        loc: Loc::from_string("3, 3").expect("Failed to create Loc from string"),
    };
    assert!(black_pass == false);
    assert!(white_pass == false);
    mv.pass(&mut black_pass, &mut 0, &mut white_pass, &mut 0);
    assert!(black_pass == true);
    assert!(white_pass == false);
    mv.player = Player::White;
    mv.pass(&mut black_pass, &mut 0, &mut white_pass, &mut 0);
    assert!(black_pass == true);
    assert!(white_pass == true);
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::board::Board;
    use crate::board::Color;
    use crate::board::Loc;
    use crate::board::Move;
    use crate::board::Player;

    #[test]
    fn stones_have_to_be_placed_on_empty_fields() {
        let mut board = Board::new(5, 5, 0);
        assert_eq!(board.fields[1][1], Color::Empty);
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        });
        board.print_board();
        assert_eq!(board.fields[1][1], Color::Black);
        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 1, col: 1 },
        });
        board.print_board();
        assert_eq!(board.fields[1][1], Color::Black);
        assert_eq!(board.fields[1][2], Color::Empty);
        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 1, col: 2 },
        });
        assert_eq!(board.fields[1][1], Color::Black);
        assert_eq!(board.fields[1][2], Color::White);
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 2 },
        });
        assert_eq!(board.fields[1][1], Color::Black);
        assert_eq!(board.fields[1][2], Color::White);
    }

    #[test]
    fn stones_are_grouped_correctly() {
        let mut board = Board::new(11, 11, 2);

        let black_groups: Vec<Loc> = vec![
            // Group 1
            Loc::from_string("1, 1").expect("Failed to create Loc from string"),
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
    }

    #[test]
    fn liberties_are_calculated_correctly() {
        let mut board = Board::new(11, 11, 2);

        let black_groups: Vec<Loc> = vec![
            // Group 1
            Loc::from_string("1, 1").expect("Failed to create Loc from string"),
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
    }

    #[test]
    fn groups_are_removed_correctly() {
        let mut board = Board::new(11, 11, 2);

        let black_groups: Vec<Loc> = vec![
            // Group 1
            Loc::from_string("1, 1").expect("Failed to create Loc from string"),
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

        board.remove_group(Loc { row: 1, col: 1 });
        assert!(board.fields[1][1] == Color::Empty);
        assert!(board.fields[1][2] == Color::Empty);
        board.remove_group(Loc { row: 5, col: 1 });
        assert!(board.fields[4][1] == Color::Empty);
        assert!(board.fields[5][1] == Color::Empty);
        board.remove_group(Loc { row: 3, col: 4 });
        assert!(board.fields[3][3] == Color::Empty);
        assert!(board.fields[3][4] == Color::Empty);
        assert!(board.fields[4][3] == Color::Empty);
        board.remove_group(Loc { row: 6, col: 7 });
        assert!(board.fields[4][7] == Color::Empty);
        assert!(board.fields[5][7] == Color::Empty);
        assert!(board.fields[6][7] == Color::Empty);
        board.remove_group(Loc { row: 3, col: 2 });
        assert!(board.fields[2][2] == Color::Empty);
        assert!(board.fields[3][1] == Color::Empty);
        assert!(board.fields[3][2] == Color::Empty);
        assert!(board.fields[4][2] == Color::Empty);
        board.remove_group(Loc { row: 9, col: 1 });
        assert!(board.fields[9][1] == Color::Empty);
        board.remove_group(Loc { row: 7, col: 3 });
        assert!(board.fields[6][2] == Color::Empty);
        assert!(board.fields[6][3] == Color::Empty);
        assert!(board.fields[7][2] == Color::Empty);
        assert!(board.fields[7][3] == Color::Empty);
        assert!(board.fields[8][2] == Color::Empty);
    }

    #[test]
    fn groups_removal_is_triggered_when_their_liberties_reach_0() {
        let mut board = Board::new(11, 11, 2);

        let black_groups: Vec<Vec<Loc>> = vec![
            // Group 1
            vec![Loc { row: 1, col: 1 }, Loc { row: 1, col: 2 }],
            // Group 2
            vec![Loc { row: 4, col: 1 }, Loc { row: 5, col: 1 }],
            // Group 3
            vec![
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 },
            ],
            // Group 4
            vec![
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 },
                Loc { row: 7, col: 7 },
            ],
            // Group 5
            vec![Loc { row: 9, col: 9 }],
        ];

        let white_groups: Vec<Vec<Loc>> = vec![
            // Takes group 1
            vec![
                Loc { row: 2, col: 1 },
                Loc { row: 2, col: 2 },
                Loc { row: 1, col: 3 },
            ],
            // Takes group 2
            vec![
                Loc { row: 3, col: 1 },
                Loc { row: 4, col: 2 },
                Loc { row: 5, col: 2 },
                Loc { row: 6, col: 1 },
            ],
            // Takes group 3
            vec![
                Loc { row: 2, col: 3 },
                Loc { row: 2, col: 4 },
                Loc { row: 3, col: 2 },
                Loc { row: 3, col: 5 },
                Loc { row: 4, col: 2 },
                Loc { row: 4, col: 4 },
                Loc { row: 5, col: 3 },
            ],
            // Takes group 4
            vec![
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
            ],
            // Takes group 5
            vec![Loc { row: 8, col: 9 }, Loc { row: 9, col: 8 }],
        ];

        for group in &black_groups {
            for mv in group {
                board.play_if_move_is_valid(&Move {
                    player: Player::Black,
                    loc: *mv,
                });
            }
        }

        for (group_index, white_moves) in white_groups.iter().enumerate() {
            for (i, mv) in white_moves.iter().enumerate() {
                board.play_if_move_is_valid(&Move {
                    player: Player::White,
                    loc: *mv,
                });
                if i + 1 == white_moves.len() {
                    for loc in &black_groups[group_index] {
                        assert!(board.fields[loc.row][loc.col] == Color::Empty);
                    }
                } else {
                    for loc in &black_groups[group_index] {
                        assert!(board.fields[loc.row][loc.col] == Color::Black);
                    }
                }
            }
        }
    }

    #[test]
    fn undoing_multiple_moves_one_after_another_and_continuing_the_game_after() {
        let mut test_move_history: Vec<Move> = vec![];
        let mut rng = rand::thread_rng();
        let mut current_move = Move {
            player: Player::White,
            loc: Loc { row: 0, col: 0 },
        };

        let mut board = Board::new(7, 7, 2);
        let mut moves_left = 10;

        while moves_left > 0 {
            let row = rng.gen_range(0..7);
            let col = rng.gen_range(0..7);
            let current_move_coords = Loc { row, col };
            current_move.loc = current_move_coords;

            if board.move_is_valid(&current_move) {
                test_move_history.push(current_move.clone());
                board.play_if_move_is_valid(&current_move);
                board.change_player(&mut current_move);
                moves_left -= 1;
            }
        }

        for _ in 1..=6 {
            let last_move = test_move_history.pop().unwrap();
            assert_ne!(
                board.fields[last_move.clone().loc.row][last_move.clone().loc.col],
                Color::Empty
            );

            board = board.undo();

            assert_eq!(
                board.fields[last_move.clone().loc.row][last_move.clone().loc.col],
                Color::Empty
            );
        }

        moves_left = 6;

        while moves_left > 0 {
            let row = rng.gen_range(0..7);
            let col = rng.gen_range(0..7);
            let current_move_coords = Loc { row, col };
            current_move.loc = current_move_coords;

            if board.move_is_valid(&current_move) {
                assert_eq!(
                    board.fields[current_move.loc.row][current_move.clone().loc.col],
                    Color::Empty
                );
                board.play_if_move_is_valid(&current_move);
                assert_ne!(
                    board.fields[current_move.loc.row][current_move.loc.col],
                    Color::Empty
                );
                board.change_player(&mut current_move);

                println!();
                moves_left -= 1;
            }
        }
    }

    #[test]
    fn undo_restores_both_groups_that_were_captured_by_the_undone_move() {
        let mut board = Board::new(7, 5, 2);

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

        board = board.undo();

        assert!(board.fields[1][1] == Color::Empty);
        assert!(board.fields[2][1] == Color::Empty);
        assert!(board.fields[3][1] == Color::White);
    }

    #[test]
    fn board_position_cannot_be_repeated() {
        let mut board = Board::new(6, 5, 2);

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
        }

        board.play_if_move_is_valid(&Move {
            player: Player::White,
            loc: Loc { row: 2, col: 1 },
        });

        assert!(board.fields[2][1] == Color::Empty);
        assert!(board.fields[1][1] == Color::Black);

        board.play_if_move_is_valid(&Move {
            player: Player::White,
            loc: Loc { row: 4, col: 3 },
        });
        board.play_if_move_is_valid(&Move {
            player: Player::Black,
            loc: Loc { row: 3, col: 3 },
        });
        board.play_if_move_is_valid(&Move {
            player: Player::White,
            loc: Loc { row: 2, col: 1 },
        });
        board.play_if_move_is_valid(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        });

        assert!(board.fields[1][1] == Color::Empty);
        assert!(board.fields[2][1] == Color::White);

        board.play_if_move_is_valid(&Move {
            player: Player::Black,
            loc: Loc { row: 2, col: 3 },
        });
        board.play_if_move_is_valid(&Move {
            player: Player::White,
            loc: Loc { row: 4, col: 2 },
        });
        board.play_if_move_is_valid(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        });

        assert!(board.fields[2][1] == Color::Empty);
        assert!(board.fields[1][1] == Color::Black);
    }

    #[test]
    fn each_group_points_are_counted_correctly() {
        let mut board = Board::new(8, 8, 0);
        let black_groups = [
            Loc { row: 1, col: 2 },
            Loc { row: 1, col: 3 },
            Loc { row: 1, col: 5 },
            Loc { row: 2, col: 1 },
            Loc { row: 2, col: 3 },
            Loc { row: 3, col: 1 },
            Loc { row: 3, col: 3 },
            Loc { row: 4, col: 2 },
            Loc { row: 5, col: 2 },
            Loc { row: 6, col: 2 },
        ];

        let white_groups = [
            Loc { row: 1, col: 4 },
            Loc { row: 2, col: 4 },
            Loc { row: 2, col: 5 },
            Loc { row: 2, col: 6 },
            Loc { row: 3, col: 4 },
            Loc { row: 4, col: 4 },
            Loc { row: 5, col: 1 },
            Loc { row: 5, col: 4 },
            Loc { row: 6, col: 4 },
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

        let loc_of_points_to_calculate = [
            Loc { row: 1, col: 1 },
            Loc { row: 1, col: 6 },
            Loc { row: 2, col: 2 },
            Loc { row: 3, col: 2 },
            Loc { row: 3, col: 5 },
            Loc { row: 3, col: 6 },
            Loc { row: 4, col: 1 },
            Loc { row: 4, col: 3 },
            Loc { row: 4, col: 5 },
            Loc { row: 4, col: 6 },
            Loc { row: 5, col: 3 },
            Loc { row: 5, col: 5 },
            Loc { row: 5, col: 6 },
            Loc { row: 6, col: 1 },
            Loc { row: 6, col: 3 },
            Loc { row: 6, col: 5 },
            Loc { row: 6, col: 6 },
        ];

        let expected_points = [
            (Color::Black, 1),
            (Color::Empty, 0),
            (Color::Black, 2),
            (Color::Black, 2),
            (Color::White, 8),
            (Color::White, 8),
            (Color::Empty, 0),
            (Color::Empty, 0),
            (Color::White, 8),
            (Color::White, 8),
            (Color::Empty, 0),
            (Color::White, 8),
            (Color::White, 8),
            (Color::Empty, 0),
            (Color::Empty, 0),
            (Color::White, 8),
            (Color::White, 8),
        ];

        for (i, loc) in loc_of_points_to_calculate.iter().enumerate() {
            assert_eq!(board.count_potential_points(*loc), expected_points[i]);
        }
    }

    #[test]
    fn board_points_are_counted_correctly() {
        let mut board = Board::new(8, 8, 0);
        let black_groups = [
            Loc { row: 1, col: 2 },
            Loc { row: 1, col: 3 },
            Loc { row: 1, col: 5 },
            Loc { row: 2, col: 1 },
            Loc { row: 2, col: 3 },
            Loc { row: 3, col: 1 },
            Loc { row: 3, col: 3 },
            Loc { row: 4, col: 2 },
            Loc { row: 5, col: 2 },
            Loc { row: 6, col: 2 },
        ];

        let white_groups = [
            Loc { row: 1, col: 4 },
            Loc { row: 2, col: 4 },
            Loc { row: 2, col: 5 },
            Loc { row: 2, col: 6 },
            Loc { row: 3, col: 4 },
            Loc { row: 4, col: 4 },
            Loc { row: 5, col: 1 },
            Loc { row: 5, col: 4 },
            Loc { row: 6, col: 4 },
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

        assert_eq!(board.count_board_points(), (3, 8));
    }
}
