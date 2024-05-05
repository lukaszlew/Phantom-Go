use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
enum Field {
    White,
    Black,
    Empty,
    Invalid,
}

impl Field {
    fn to_string(&self) -> String {
        match &self {
            Field::Empty => ".".into(),
            Field::White => "O".into(),
            Field::Black => "#".into(),
            Field::Invalid => "/".into(),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Player {
    White,
    Black,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Loc {
    row: usize,
    col: usize,
}

// impl Loc {
//     fn up(&self) -> Self {
//         Loc {
//             row: self.row - 1,
//             col: self.col,
//         }
//     }
//     fn down(&self) -> Self {
//         Loc {
//             row: self.row + 1,
//             col: self.col,
//         }
//     }
//     fn left(&self) -> Self {
//         Loc {
//             row: self.row,
//             col: self.col - 1,
//         }
//     }
//     fn right(&self) -> Self {
//         Loc {
//             row: self.row,
//             col: self.col + 1,
//         }
//     }
// }

struct Move {
    player: Player,
    coords: Loc,
}

struct Board {
    fields: Vec<Vec<Field>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        // Initializing an empty board
        let mut board = Board {
            fields: vec![vec![Field::Empty; cols]; rows],
        };
        // Setting up sentinels in rows
        for i in 0..cols {
            board.fields[0][i] = Field::Invalid;
            board.fields[rows - 1][i] = Field::Invalid;
        }
        // Setting up sentinels in columns
        for i in 0..rows {
            board.fields[i][0] = Field::Invalid;
            board.fields[i][cols - 1] = Field::Invalid;
        }
        board
    }

    fn print_board(&self) {
        for row in &self.fields {
            for cell in row {
                print!("{} ", cell.to_string());
            }
            println!();
        }
    }

    fn move_is_valid(&self, move_coords: &Loc) -> bool {
        self.fields[move_coords.row][move_coords.col] == Field::Empty
    }

    fn play(&mut self, current_move: &Move) {
        match current_move.player {
            Player::Black => {
                println!("Black made a move!\n");
                self.fields[current_move.coords.row][current_move.coords.col] = Field::Black;
            }
            Player::White => {
                println!("White made a move!\n");
                self.fields[current_move.coords.row][current_move.coords.col] = Field::White;
            }
        }
    }

    // fn group_stones(&self, coords: Loc, color: Field, stone_groups: &mut Vec<Vec<Loc>>) {
    //     let mut current_group: Vec<Loc> = vec![];
    //     let current_color = &self.fields[coords.row][coords.col];
    //     flood_fill(self, coords, &color, current_color, &mut current_group);
    //     stone_groups.push(current_group);
    // }

    // fn count_liberties(stone_from_group_coord: &Loc) {}

    // fn remove_group(stone_from_group_coord: &Loc) {}
}

// fn flood_fill(
//     board: &Board,
//     coords: Loc,
//     color: &Field,
//     current_color: &Field,
//     stone_groups: &mut Vec<Loc>,
// ) {
//     // Check if out of bounds
//     if board.fields[coords.row][coords.col] == Field::Invalid {
//         return;
//     }
//     // Check if we're not our connected stone position
//     if current_color != &board.fields[coords.row][coords.col] {
//         return;
//     }
//     // Check if we've already visited this coord
//     if stone_groups.contains(&coords) {
//         return;
//     }

//     stone_groups.push(coords.clone());

//     flood_fill(board, coords.up(), color, current_color, stone_groups);
//     flood_fill(board, coords.down(), color, current_color, stone_groups);
//     flood_fill(board, coords.left(), color, current_color, stone_groups);
//     flood_fill(board, coords.right(), color, current_color, stone_groups);
// }

// fn remove_duplicates(stone_groups: &mut Vec<Vec<Loc>>) {
//     let mut unique_groups: Vec<Vec<Loc>> = Vec::new();

//     for i in 0..stone_groups.len() {
//         let mut is_subset = false;
//         for j in 0..stone_groups.len() {
//             if i != j
//                 && stone_groups[i]
//                     .iter()
//                     .all(|loc| stone_groups[j].contains(loc))
//             {
//                 is_subset = true;
//                 break;
//             }
//         }
//         if !is_subset {
//             unique_groups.push(stone_groups[i].clone());
//         }
//     }

//     *stone_groups = unique_groups;
// }

fn main() {
    let mut board = Board::new(7, 7);
    let mut rng = rand::thread_rng();
    // let mut stone_groups = vec![vec![]];
    let mut current_move = Move {
        player: Player::White,
        coords: Loc { row: 0, col: 0 },
    };

    let mut moves_left = 10;
    board.print_board();

    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = Loc { row, col };
        println!();

        if board.move_is_valid(&current_move_coords) {
            current_move = Move {
                player: match current_move.player {
                    Player::Black => Player::White,
                    Player::White => Player::Black,
                },
                coords: current_move_coords,
            };
            board.play(&current_move);
            board.print_board();

            // // After making a move, identify groups of stones
            // let current_color = match current_move.player {
            //     Player::Black => Field::Black,
            //     Player::White => Field::White,
            // };

            // board.group_stones(current_move.coords, current_color, &mut stone_groups);

            moves_left -= 1;
        } else {
            println!("Invalid move :c");
        }
        println!("Moves left: {}", moves_left);
    }

    // remove_duplicates(&mut stone_groups);

    // // Iterate over groups of stones and print their coordinates
    // for (group_index, group) in stone_groups.iter().enumerate() {
    //     println!(
    //         "\nGroup #{}\nStone coordinates: {:?}",
    //         group_index + 1,
    //         group
    //     );
    // }
    println!("\nF I N A L  B O A R D:\n");
    board.print_board();
}
