use rand::Rng;
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
    Empty,
    Invalid,
}

impl Color {
    fn to_string(&self) -> String {
        match &self {
            Color::Empty => ".".into(),
            Color::White => "O".into(),
            Color::Black => "#".into(),
            Color::Invalid => "/".into(),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Player {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Loc {
    row: usize,
    col: usize,
}

impl Loc {
    fn up(&self) -> Self {
        Loc {
            row: self.row - 1,
            col: self.col,
        }
    }
    fn down(&self) -> Self {
        Loc {
            row: self.row + 1,
            col: self.col,
        }
    }
    fn left(&self) -> Self {
        Loc {
            row: self.row,
            col: self.col - 1,
        }
    }
    fn right(&self) -> Self {
        Loc {
            row: self.row,
            col: self.col + 1,
        }
    }
    fn from_string(s: &str) -> Self {
        let row_col: Vec<&str> = s.split(",").map(|part| part.trim()).collect();

        let row = row_col[0].parse::<usize>().unwrap_or(0);
        let col = row_col[1].parse::<usize>().unwrap_or(0);

        Loc { row, col }
    }
}

struct Move {
    player: Player,
    loc: Loc,
}

#[derive(Clone)]
struct Board {
    fields: Vec<Vec<Color>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        // Initializing an empty board
        let mut board = Board {
            fields: vec![vec![Color::Empty; cols]; rows],
        };
        // Setting up sentinels in rows
        for i in 0..cols {
            board.fields[0][i] = Color::Invalid;
            board.fields[rows - 1][i] = Color::Invalid;
        }
        // Setting up sentinels in columns
        for i in 0..rows {
            board.fields[i][0] = Color::Invalid;
            board.fields[i][cols - 1] = Color::Invalid;
        }
        board
    }

    fn get(&self, loc: Loc) -> Color {
        self.fields[loc.row][loc.col]
    }

    fn set(&mut self, loc: Loc, color: Color) {
        self.fields[loc.row][loc.col] = color;
    }

    fn print_board(&self) {
        for row in &self.fields {
            for cell in row {
                print!("{} ", cell.to_string());
            }
            println!();
        }
    }

    fn field_is_empty(&self, loc: Loc) -> bool {
        self.get(loc) == Color::Empty
    }

    fn change_player(&self, mv: &mut Move) {
        mv.player = match mv.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    fn move_is_valid(&self, mv: &Move) -> bool {
        let mut potential_board = self.clone();
        if potential_board.field_is_empty(mv.loc) {
            potential_board.play(mv);
        } else {
            return false;
        }
        potential_board.count_liberties(mv.loc) > 0
    }

    fn play(&mut self, current_move: &Move) {
        match current_move.player {
            Player::Black => {
                self.fields[current_move.loc.row][current_move.loc.col] = Color::Black;
            }
            Player::White => {
                self.fields[current_move.loc.row][current_move.loc.col] = Color::White;
            }
        }
        self.remove_groups_after_move(current_move.loc);
    }

    fn group_stones(&self, loc: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[loc.row][loc.col];
        self.flood_fill(loc, color, &mut group_stones_coordinates);
        custom_sort(group_stones_coordinates)
    }

    fn flood_fill(&self, loc: Loc, color: Color, visited: &mut Vec<Loc>) {
        if visited.contains(&loc) {
            return;
        }
        if self.get(loc) != color {
            return;
        }

        visited.push(loc);

        self.flood_fill(loc.up(), color, visited);
        self.flood_fill(loc.down(), color, visited);
        self.flood_fill(loc.left(), color, visited);
        self.flood_fill(loc.right(), color, visited);
    }

    fn count_liberties(&self, loc: Loc) -> usize {
        let group = self.group_stones(loc);
        let mut liberties: HashSet<Loc> = HashSet::new();
        fn get_check_empty_insert_combo(board: &Board, loc: Loc, liberties: &mut HashSet<Loc>) {
            let color = board.get(loc);
            if color == Color::Empty {
                liberties.insert(loc);
            }
        }
        for stone_coords in group {
            get_check_empty_insert_combo(self, stone_coords.up(), &mut liberties);
            get_check_empty_insert_combo(self, stone_coords.down(), &mut liberties);
            get_check_empty_insert_combo(self, stone_coords.left(), &mut liberties);
            get_check_empty_insert_combo(self, stone_coords.right(), &mut liberties);
        }
        liberties.len()
    }

    fn remove_group(&mut self, loc: Loc) {
        let group = self.group_stones(loc);
        for stone in group {
            self.set(stone, Color::Empty);
        }
    }

    fn remove_groups_after_move(&mut self, loc: Loc) {
        fn get_check_invalid_remove_group_combo(board: &mut Board, loc: Loc) {
            let color = board.get(loc);
            let color_is_correct = color != Color::Invalid && color != Color::Empty;
            if color_is_correct {
                let group_liberties_is_0 = board.count_liberties(loc) == 0;
                if group_liberties_is_0 {
                    board.remove_group(loc);
                }
            }
        }
        get_check_invalid_remove_group_combo(self, loc.up());
        get_check_invalid_remove_group_combo(self, loc.down());
        get_check_invalid_remove_group_combo(self, loc.left());
        get_check_invalid_remove_group_combo(self, loc.right());
    }
}

fn custom_sort(mut group: Vec<Loc>) -> Vec<Loc> {
    group.sort_by(|a, b| {
        if a.row > b.row {
            return Ordering::Greater;
        } else if a.row < b.row {
            return Ordering::Less;
        } else if a.col > b.col {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    });
    group
}

fn run_tests(mut board: Board) {
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
        board.play(&Move {
            player: Player::Black,
            loc: mv,
        })
    }

    for mv in white_groups {
        board.play(&Move {
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
        board.play(&Move {
            player: Player::Black,
            loc: mv,
        });
    }

    board.print_board();

    for mv in white_groups {
        board.play(&Move {
            player: Player::White,
            loc: mv,
        });
        println!("After trying to remove a group after {:?} move", mv);
        board.print_board();
    }
}

fn main() {
    let board = Board::new(11, 11);
    println!();
    run_tests(board);
    println!("\nAll tests P A S S E D !\n");

    let mut rng = rand::thread_rng();
    let mut current_move = Move {
        player: Player::Black,
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
            board.play(&current_move);
            board.change_player(&mut current_move);

            moves_left -= 1;
        }
    }

    println!("\nF I N A L  B O A R D:\n");
    board.print_board();
    println!("{:?}", Loc::from_string("1, 9"));
}
