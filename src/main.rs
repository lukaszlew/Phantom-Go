use rand::Rng;
use std::cmp::Ordering;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}

struct Move {
    player: Player,
    loc: Loc,
}

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

    fn print_board(&self) {
        for row in &self.fields {
            for cell in row {
                print!("{} ", cell.to_string());
            }
            println!();
        }
    }

    fn move_is_valid(&self, loc: Loc) -> bool {
        self.get(loc) == Color::Empty
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
    }

    fn group_stones(&self, loc: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[loc.row][loc.col];
        self.flood_fill(loc, color, &mut group_stones_coordinates);
        group_stones_coordinates
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

    // fn count_liberties(stone_from_group_coord: &Loc) {}

    // fn remove_group(stone_from_group_coord: &Loc) {}
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
    assert!(custom_sort(group1_a) == [Loc { row: 1, col: 1 }, Loc { row: 1, col: 2 }]);
    assert!(custom_sort(group1_b) == [Loc { row: 1, col: 1 }, Loc { row: 1, col: 2 }]);
    assert!(custom_sort(group2_a) == [Loc { row: 4, col: 1 }, Loc { row: 5, col: 1 }]);
    assert!(custom_sort(group2_b) == [Loc { row: 4, col: 1 }, Loc { row: 5, col: 1 }]);
    assert!(
        custom_sort(group3_a)
            == [
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        custom_sort(group3_b)
            == [
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        custom_sort(group3_c)
            == [
                Loc { row: 3, col: 3 },
                Loc { row: 3, col: 4 },
                Loc { row: 4, col: 3 }
            ]
    );
    assert!(
        custom_sort(group4_a)
            == [
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        custom_sort(group4_b)
            == [
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        custom_sort(group4_c)
            == [
                Loc { row: 4, col: 7 },
                Loc { row: 5, col: 7 },
                Loc { row: 6, col: 7 }
            ]
    );
    assert!(
        custom_sort(group5_a)
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        custom_sort(group5_b)
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        custom_sort(group5_c)
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(
        custom_sort(group5_d)
            == [
                Loc { row: 2, col: 2 },
                Loc { row: 3, col: 1 },
                Loc { row: 3, col: 2 },
                Loc { row: 4, col: 2 }
            ]
    );
    assert!(custom_sort(group6) == [Loc { row: 9, col: 1 }]);
    assert!(
        custom_sort(group7_a)
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        custom_sort(group7_b)
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        custom_sort(group7_c)
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        custom_sort(group7_d)
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
    assert!(
        custom_sort(group7_e)
            == [
                Loc { row: 6, col: 2 },
                Loc { row: 6, col: 3 },
                Loc { row: 7, col: 2 },
                Loc { row: 7, col: 3 },
                Loc { row: 8, col: 2 }
            ]
    );
}

fn main() {
    let board = Board::new(11, 11);
    println!();
    run_tests(board);
    println!("\nAll tests P A S S E D !\n");

    let mut rng = rand::thread_rng();
    let mut current_move = Move {
        player: Player::White,
        loc: Loc { row: 0, col: 0 },
    };

    let mut board = Board::new(7, 7);
    let mut moves_left = 10;
    board.print_board();

    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = Loc { row, col };

        if board.move_is_valid(current_move_coords) {
            current_move = Move {
                player: match current_move.player {
                    Player::Black => Player::White,
                    Player::White => Player::Black,
                },
                loc: current_move_coords,
            };
            board.play(&current_move);
            println!();
            board.print_board();

            moves_left -= 1;
        }
    }

    println!("\nF I N A L  B O A R D:\n");
    board.print_board();
}
