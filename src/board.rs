use std::collections::HashSet;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            Color::White => "#".into(),
            Color::Black => "O".into(),
            Color::Invalid => "/".into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
pub enum Winner {
    White,
    Black,
    Draw,
}

impl Player {
    pub fn to_color(&self) -> Color {
        match self {
            Player::Black => Color::Black,
            Player::White => Color::White,
        }
    }

    pub fn change(self) -> Self { 
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
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

    pub fn from_string(s: &str) -> Option<Self> {
        let mut loc = Loc { row: 0, col: 0 };  // TODO evil style. Remove this line.
        // if input doesn't have a comma - definitely invalid
        if !s.contains(",") {
            return None;
        }

        let row_col: Vec<&str> = s.split(",").map(|part| part.trim()).collect();
        // if has more than 1 - definitely invalid
        if row_col.len() != 2 {
            return None;
        }

        loc.row = row_col[0].parse::<usize>().unwrap_or(0); // TODO bug, if error you want to retunrn None. Learn about question mark.
        loc.col = row_col[1].parse::<usize>().unwrap_or(0);

        Some(loc)
    }

    fn is_on_board(&self, board_size: (usize, usize)) -> bool {
        let upper_edge_check = self.row > 0;
        let lower_edge_check = self.row < board_size.0 - 1;
        let left_edge_check = self.col > 0;
        let right_edge_check = self.col < board_size.1 - 1;

        upper_edge_check && lower_edge_check && left_edge_check && right_edge_check
    }

    fn coords(&self, rows: usize) -> String {  // TODO: del unused code.
        let col = "ABCDEFGHJKLMNOPQRST";
        let row = rows - self.row;
        let col = col.chars().nth(self.col).unwrap();
        return format!("{}{}", col, row);
    }

    // Checking borders for each "island"
    fn get_bordering_colors(&self, island: &Vec<Loc>, board: &Board) -> HashSet<Color> {  
        // TODO: Bad style, Loc should not need to know about Board. Move to Board.
        //   self is not even used :D
        let mut bordering_colors: HashSet<Color> = HashSet::new();
        for field in island {
            bordering_colors.insert(board.get(field.up()));
            bordering_colors.insert(board.get(field.down()));
            bordering_colors.insert(board.get(field.left()));
            bordering_colors.insert(board.get(field.right()));
        }
        bordering_colors
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub player: Player,
    pub loc: Loc,
}

impl Move {
    pub fn pass(self) -> Self {
        Move {
            player: self.player,
            loc: Loc { row: 99, col: 99 },
        }
    }

    pub fn is_pass(&self) -> bool {
        match self.loc {
            Loc { row: 99, col: 99 } => true,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    fields: Vec<Vec<Color>>,
    // TODO: work towards making ALL field private
    pub game_history: Vec<Move>,
    komi: f32,
    pub black_captures: isize,
    pub white_captures: isize,
    result: f32,
}

impl Board {
    pub fn new(rows: usize, cols: usize, komi: f32) -> Self {
        // Initializing an empty board
        let mut board = Board {
            fields: vec![vec![Color::Empty; cols]; rows],
            game_history: vec![],
            komi,
            black_captures: 0,
            white_captures: 0,
            result: 0.0,
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

    pub fn reset(&self) -> Self {
        Board::new(self.fields.len(), self.fields[0].len(), self.komi)
    }

    pub fn get(&self, loc: Loc) -> Color {
        self.fields[loc.row][loc.col]
    }

    fn set(&mut self, loc: Loc, color: Color) {
        self.fields[loc.row][loc.col] = color;
    }

    pub fn to_string(&self) -> String {
        let mut board_string = String::new();
        for row in &self.fields {
            for field in row {
                board_string.push_str(&format!("{} ", field.to_string()));
            }
            board_string += "\n";
        }
        board_string
    }

    fn board_size(&self) -> (usize, usize) {
        (self.fields.len(), self.fields[0].len())
    }

    fn get_all_loc(&self) -> Vec<Loc> {  // TODO: Loc.all((usize, usize)) -> Vec<Loc>
        let mut all_loc: Vec<Loc> = vec![];
        for (i, row) in self.fields.iter().enumerate() {
            for j in 0..row.len() {
                all_loc.push(Loc { row: i, col: j })
            }
        }
        all_loc
    }
    
    // Creates a set of potential points - "islands" of Color::Empty
    // TODO: comments often suggest renaming. E.g., 
    // Lingo "island" - set of locs of empty fields ...
    // empty_islands(&self) -> HashSet<Vec<Loc>>
    fn create_set_of_potential_points(&self) -> HashSet<Vec<Loc>> {
        // TODO let mut islands = ...
        let mut groups_of_empty: HashSet<Vec<Loc>> = HashSet::new();
        for loc in self.get_all_loc() {
            if self.get(loc) == Color::Empty {
                // If the group of Locs contains current Loc, the group of this loc has already been added
                if !groups_of_empty.iter().any(|group| group.contains(&loc)) {
                    groups_of_empty.insert(self.group_stones(loc));
                }
            }
        }
        groups_of_empty
    }

    // TODO: Not pub!
    pub fn count_potential_points(&self, loc: Loc) -> (Color, isize) {
        if self.get(loc) != Color::Empty {
            return (Color::Invalid, 0);
        }

        let group = self.group_stones(loc);
        let bordering_colors = loc.get_bordering_colors(&group, &self);

        let potential_points_border_both_colors =
            bordering_colors.contains(&Color::Black) && bordering_colors.contains(&Color::White);
        let board_is_empty =
            bordering_colors.len() == 1 && bordering_colors.contains(&Color::Invalid);
        let potential_points_are_dame = potential_points_border_both_colors || board_is_empty;

        let mut player_and_points: (Color, isize) = (Color::Empty, 0);

        if !potential_points_are_dame {
            let points: isize = group.len().try_into().unwrap();
            if bordering_colors.contains(&Color::Black) {
                player_and_points = (Color::Black, points);
            }
            if bordering_colors.contains(&Color::White) {
                player_and_points = (Color::White, points);
            }
        }

        player_and_points
    }

    // Grouping empty "islands" and checking bordering Colors to decide which Color the points belong
    pub fn count_board_points(&self) -> (isize, isize) {
        // Populating the HashSet of Empty "islands"
        let groups_of_potential_points = self.create_set_of_potential_points();

        let mut white_points: isize = 0;
        let mut black_points: isize = 0;

        for potential_points in groups_of_potential_points {
            let color_and_points = self.count_potential_points(potential_points[0]);
            match color_and_points.0 {
                Color::Black => black_points += color_and_points.1,
                Color::White => white_points += color_and_points.1,
                Color::Empty => (),
                Color::Invalid => (),
            }
        }

        (black_points, white_points)
    }

    pub fn count_score(&self) -> (Winner, f32) {
        let all_points = self.count_board_points();
        let black_total_points: f32 = all_points.0 as f32 + self.black_captures as f32;
        let white_total_points: f32 = all_points.1 as f32 + self.white_captures as f32 + self.komi;

        if black_total_points - white_total_points == 0.0 {
            return (Winner::Draw, 0.0);
        }

        let black_won = black_total_points > white_total_points;
        let result: (Winner, f32);

        if black_won {
            result = (Winner::Black, black_total_points - white_total_points);
        } else {
            result = (Winner::White, white_total_points - black_total_points)
        }

        result
    }

    pub fn print_result(&self) {
        let score = self.count_score();
        match score.0 {
            Winner::Draw => println!("\nD R A W !"),
            _ => println!("{:?} won by {:?}!", score.0, score.1),
        }
    }

    pub fn board_position_is_reapeated(&self, board: Board) -> bool {
        self.fields == board.fields
    }

    pub fn move_is_valid(&self, mv: &Move) -> bool {
        let board_size = self.board_size();
        if !mv.loc.is_on_board(board_size) {
            return false;
        }
        let mut potential_board = self.clone();
        if potential_board.get(mv.loc) == Color::Empty {
            potential_board.play(mv);
        } else {
            return false;
        }

        let mut gh_copy = self.game_history.clone();
        gh_copy.pop();
        let mut board_from_2_moves_ago = self.reset();
        for mv in gh_copy {
            board_from_2_moves_ago.play(&mv);
        }
        // If the group has been removed after the move, it was a suicidcal move
        let move_is_suicidal = potential_board.get(mv.loc) == Color::Empty;
        let board_is_repeated = board_from_2_moves_ago.board_position_is_reapeated(potential_board);

        !move_is_suicidal && !board_is_repeated
    }

    pub fn play(&mut self, mv: &Move) {
        if mv.is_pass() {
            self.game_history.push(mv.clone());
            return;
        }
        if self.get(mv.loc) == Color::Empty {
            self.set(mv.loc, mv.player.to_color());
        }
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
        // update game_history
        self.game_history.push(mv.clone());
    }

    pub fn play_if_move_is_valid(&mut self, mv: &Move) {
        if self.move_is_valid(mv) {
            self.play(mv);
        }
    }

    pub fn group_stones(&self, loc: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[loc.row][loc.col];
        self.flood_fill(loc, color, &mut group_stones_coordinates);
        group_stones_coordinates.sort_by(|a, b| a.row.cmp(&b.row).then(a.col.cmp(&b.col)));
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

    pub fn count_liberties(&self, loc: Loc) -> usize {
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

    pub fn remove_group(&mut self, loc: Loc) {
        let group = self.group_stones(loc);
        let stone_count: isize = group.len().try_into().unwrap();
        match self.get(loc) {
            Color::White => self.black_captures += stone_count,
            Color::Black => self.white_captures += stone_count,
            _ => (),
        }
        for stone in group {
            self.set(stone, Color::Empty);
        }
    }

    pub fn undo(mut self) -> Self {
        self.game_history.pop();
        let mut board_after_undo = Board::new(self.fields.len(), self.fields[0].len(), self.komi);
        for mv in &self.game_history {
            board_after_undo.play(mv);
        }
        board_after_undo
    }
}

pub fn take_player_input() -> String {
    let mut player_input = String::new();
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input");
    player_input = player_input.trim().to_string();
    player_input
}
