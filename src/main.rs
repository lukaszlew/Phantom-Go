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

#[derive(PartialEq)]
enum Player {
    White,
    Black,
}
#[derive(Debug, Clone, PartialEq)]
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
    fields: Vec<Vec<Field>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        // Initializing an empty board
        let mut board = Board {
            fields: vec![vec![Field::Empty; rows]; cols],
        };
        // Setting up sentinels
        for i in 0..rows {
            board.fields[0][i] = Field::Invalid;
            board.fields[rows - 1][i] = Field::Invalid;
            board.fields[i][0] = Field::Invalid;
            board.fields[i][cols - 1] = Field::Invalid;
        }
        board
    }

    fn print_board(board: &Self) {
        for row in &board.fields {
            for cell in row {
                print!("{} ", cell.to_string());
            }
            println!();
        }
    }

    fn move_is_valid(board: &Self, move_coords: &Loc) -> bool {
        // https://stackoverflow.com/a/51429606
        // https://doc.rust-lang.org/std/macro.matches.html
        if matches!(
            &board.fields[move_coords.row][move_coords.col],
            Field::Black | Field::White | Field::Invalid
        ) {
            false
        } else {
            true
        }
    }

    fn play<'a>(
        board: &'a mut Self,
        move_coords: &Loc,
        player_color: &'a mut Player,
    ) -> &'a mut Board {
        println!();
        match player_color {
            Player::Black => {
                println!("Black made a move!\n");
                board.fields[move_coords.row][move_coords.col] = Field::Black;
                // player_color = Player::White, attempts to reassign the entire player_color reference to a new value,
                // which is not allowed because Rust enforces ownership rules and doesn't allow reassigning references.
                // To modify a value inside a mutable reference, you need to dereference the reference first using the * operator.
                *player_color = Player::White;
            }
            Player::White => {
                println!("White made a move!\n");
                board.fields[move_coords.row][move_coords.col] = Field::White;
                // player_color = Player::White, attempts to reassign the entire player_color reference to a new value,
                // which is not allowed because Rust enforces ownership rules and doesn't allow reassigning references.
                // To modify a value inside a mutable reference, you need to dereference the reference first using the * operator.
                *player_color = Player::Black;
            }
        }
        board
    }
}

fn group_stones(board: &Board, coords: Loc, color: Field) -> Vec<Loc> {
    let mut board = board;
    let current_color = &board.fields[coords.row][coords.col];
    // didn't have a better idea, so I used coords.clone()
    let mut group_stones_coords: Vec<Loc> = vec![coords.clone()];
    flood_fill(
        &mut board,
        coords,
        &color,
        current_color,
        &mut group_stones_coords,
    );
    group_stones_coords
}

fn flood_fill(
    board: &Board,
    coords: Loc,
    color: &Field,
    current_color: &Field,
    group_stones_coords: &mut Vec<Loc>,
) {
    // Check if out of bounds
    if coords.row >= board.fields.len() || coords.col >= board.fields[0].len() {
        return;
    }
    // Check if we're not our connected stone position
    if current_color != &board.fields[coords.row][coords.col] {
        return;
    }
    // Check if we've already visited this coord
    if group_stones_coords.contains(&coords) {
        return;
    }

    group_stones_coords.push(coords.clone());

    flood_fill(
        board,
        coords.up(),
        color,
        current_color,
        group_stones_coords,
    );
    flood_fill(
        board,
        coords.down(),
        color,
        current_color,
        group_stones_coords,
    );
    flood_fill(
        board,
        coords.left(),
        color,
        current_color,
        group_stones_coords,
    );
    flood_fill(
        board,
        coords.right(),
        color,
        current_color,
        group_stones_coords,
    );
}

fn main() {
    let mut board = Board::new(7, 7);
    let mut player = Player::Black;
    let mut rng = rand::thread_rng();
    let mut moves_left = 10;
    Board::print_board(&board);
    while moves_left > 0 {
        let row = rng.gen_range(0..7);
        let col = rng.gen_range(0..7);
        let current_move_coords = Loc { row, col };
        println!();
        if Board::move_is_valid(&board, &current_move_coords) {
            Board::play(&mut board, &current_move_coords, &mut player);
            Board::print_board(&board);
            moves_left -= 1;
        } else {
            println!("Invalid move :c");
        }
        println!("Moves left: {}", moves_left);
    }
    println!("\nF I N A L  B O A R D:\n");
    Board::print_board(&board);
}
