use rand::Rng;
use std::collections::HashSet;
pub mod board_test;

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

#[derive(Clone)]
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

    fn change_player(&self, mv: &mut Move) {
        mv.player = match mv.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    fn move_is_valid(&self, mv: &Move) -> bool {
        let mut potential_board = self.clone();
        if potential_board.get(mv.loc) == Color::Empty {
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
        // Remove dead groups
        fn get_check_invalid_remove_group_combo(board: &mut Board, loc: Loc) {
            let color = board.get(loc);
            if color != Color::Invalid && color != Color::Empty {
                let group_liberties_is_0 = board.count_liberties(loc) == 0;
                if group_liberties_is_0 {
                    board.remove_group(loc);
                }
            }
        }
        get_check_invalid_remove_group_combo(self, current_move.loc.up());
        get_check_invalid_remove_group_combo(self, current_move.loc.down());
        get_check_invalid_remove_group_combo(self, current_move.loc.left());
        get_check_invalid_remove_group_combo(self, current_move.loc.right());
    }

    fn group_stones(&self, loc: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[loc.row][loc.col];
        self.flood_fill(loc, color, &mut group_stones_coordinates);
        sort(group_stones_coordinates)
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

    fn undo(mut self, moves: &mut Vec<Move>) -> Self {
        moves.pop();
        self = Board::new(7, 7);
        for mv in moves {
            self.fields[mv.loc.row][mv.loc.col] = match mv.player {
                Player::Black => Color::Black,
                Player::White => Color::White,
            };
        }
        self
    }
}

fn sort(mut group: Vec<Loc>) -> Vec<Loc> {
    group.sort_by(|a, b| a.row.cmp(&b.row).then(a.col.cmp(&b.col)));
    group
}

fn main() {
    let board = Board::new(11, 11);
    let mut game_record: Vec<Move> = vec![];
    println!();
    board_test::run_tests(board);
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
        let current_move_coords = Loc { row, col };
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
}
