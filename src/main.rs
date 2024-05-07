use rand::Rng;

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
// Need to derive PartialEq to compare Color in flood_fill
// Need Debug to print groups of stones at line 172
#[derive(Debug, Clone, Copy, PartialEq)]
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
    coords: Loc,
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

    fn print_board(&self) {
        for row in &self.fields {
            for cell in row {
                print!("{} ", cell.to_string());
            }
            println!();
        }
    }

    fn move_is_valid(&self, move_coords: &Loc) -> bool {
        self.fields[move_coords.row][move_coords.col] == Color::Empty
    }

    fn play(&mut self, current_move: &Move) {
        match current_move.player {
            Player::Black => {
                println!("Black made a move!\n");
                self.fields[current_move.coords.row][current_move.coords.col] = Color::Black;
            }
            Player::White => {
                println!("White made a move!\n");
                self.fields[current_move.coords.row][current_move.coords.col] = Color::White;
            }
        }
    }

    fn group_stones(&self, coords: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[coords.row][coords.col];
        self.flood_fill(coords, color, &mut group_stones_coordinates);
        group_stones_coordinates
    }
    // To push to visited, it's the contents of visited that need to be mutable, not variable visited itself
    // The vector used in group_stones is mutable, because otherwise it wouldn't be possible to push to it
    fn flood_fill(&self, coords: Loc, color: Color, visited: &mut Vec<Loc>) {
        if visited.contains(&coords) {
            return;
        }
        if self.fields[coords.row][coords.col] != color {
            return;
        }

        visited.push(coords);

        self.flood_fill(coords.up(), color, visited);
        self.flood_fill(coords.down(), color, visited);
        self.flood_fill(coords.left(), color, visited);
        self.flood_fill(coords.right(), color, visited);
    }

    // fn count_liberties(stone_from_group_coord: &Loc) {}

    // fn remove_group(stone_from_group_coord: &Loc) {}
}

fn main() {
    let mut board = Board::new(7, 7);
    let mut rng = rand::thread_rng();
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
            println!("{:?}", board.group_stones(current_move_coords));

            moves_left -= 1;
        } else {
            println!("Invalid move :c");
        }
        println!("Moves left: {}", moves_left);
    }

    println!("\nF I N A L  B O A R D:\n");
    board.print_board();
}
