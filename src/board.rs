use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
    Empty,
    Invalid,
}

impl Color {
    pub fn to_string(&self) -> String {
        match &self {
            Color::Empty => ".".into(),
            Color::White => "O".into(),
            Color::Black => "#".into(),
            Color::Invalid => "/".into(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn to_color(&self) -> Color {
        match self {
            Player::Black => Color::White,
            Player::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc {
    pub row: usize,
    pub col: usize,
}

impl Loc {
    pub fn up(&self) -> Self {
        Loc {
            row: self.row - 1,
            col: self.col,
        }
    }
    pub fn down(&self) -> Self {
        Loc {
            row: self.row + 1,
            col: self.col,
        }
    }
    pub fn left(&self) -> Self {
        Loc {
            row: self.row,
            col: self.col - 1,
        }
    }
    pub fn right(&self) -> Self {
        Loc {
            row: self.row,
            col: self.col + 1,
        }
    }
    pub fn from_string(s: &str) -> Self {
        let mut loc = Loc { row: 0, col: 0 };
        // if input doesn't have a comma - definitely invalid
        if !s.contains(",") {
            return loc;
        }

        let row_col: Vec<&str> = s.split(",").map(|part| part.trim()).collect();
        // if has more than 1 - definitely invalid
        if row_col.len() != 2 {
            return loc;
        }

        loc.row = row_col[0].parse::<usize>().unwrap_or(0);
        loc.col = row_col[1].parse::<usize>().unwrap_or(0);
        println!("Inside from_string(): row: {}, col: {}", loc.row, loc.col);

        loc
    }
}

#[derive(Clone)]
pub struct Move {
    pub player: Player,
    pub loc: Loc,
}

#[derive(Clone)]
pub struct Board {
    pub fields: Vec<Vec<Color>>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
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

    pub fn get(&self, loc: Loc) -> Color {
        self.fields[loc.row][loc.col]
    }

    pub fn set(&mut self, loc: Loc, color: Color) {
        self.fields[loc.row][loc.col] = color;
    }

    pub fn print_board(&self) {
        for row in &self.fields {
            for cell in row {
                print!("{} ", cell.to_string());
            }
            println!();
        }
    }

    pub fn change_player(&self, mv: &mut Move) {
        mv.player = match mv.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn move_is_valid(&self, mv: &Move) -> bool {
        let rows = self.fields.len();
        let cols = self.fields[0].len();
        if mv.loc.row > rows - 1 || mv.loc.col > cols - 1 {
            return false;
        }
        let mut potential_board = self.clone();
        if potential_board.get(mv.loc) == Color::Empty {
            potential_board.play(mv);
        } else {
            return false;
        }
        potential_board.count_liberties(mv.loc) > 0
    }

    pub fn play(&mut self, mv: &Move) {
        self.fields[mv.loc.row][mv.loc.col] = mv.player.to_color();
        // Remove dead groups
        pub fn get_check_invalid_remove_group_combo(board: &mut Board, loc: Loc) {
            let color = board.get(loc);
            if color != Color::Invalid && color != Color::Empty && board.count_liberties(loc) == 0 {
                board.remove_group(loc);
            }
        }
        get_check_invalid_remove_group_combo(self, mv.loc.up());
        get_check_invalid_remove_group_combo(self, mv.loc.down());
        get_check_invalid_remove_group_combo(self, mv.loc.left());
        get_check_invalid_remove_group_combo(self, mv.loc.right());
    }

    pub fn group_stones(&self, loc: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[loc.row][loc.col];
        self.flood_fill(loc, color, &mut group_stones_coordinates);
        sort(group_stones_coordinates)
    }

    pub fn flood_fill(&self, loc: Loc, color: Color, visited: &mut Vec<Loc>) {
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

    pub fn count_liberties(&self, loc: Loc) -> usize {
        let group = self.group_stones(loc);
        let mut liberties: HashSet<Loc> = HashSet::new();
        pub fn get_check_empty_insert_combo(board: &Board, loc: Loc, liberties: &mut HashSet<Loc>) {
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

    pub fn remove_group(&mut self, loc: Loc) {
        let group = self.group_stones(loc);
        for stone in group {
            self.set(stone, Color::Empty);
        }
    }

    pub fn undo(mut self, moves: &mut Vec<Move>) -> Self {
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

pub fn sort(mut group: Vec<Loc>) -> Vec<Loc> {
    group.sort_by(|a, b| a.row.cmp(&b.row).then(a.col.cmp(&b.col)));
    group
}
