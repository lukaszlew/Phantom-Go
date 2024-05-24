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

#[derive(Clone, PartialEq, Debug)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn to_color(&self) -> Color {
        match self {
            Player::Black => Color::Black,
            Player::White => Color::White,
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
        let mut loc = Loc { row: 0, col: 0 };
        // if input doesn't have a comma - definitely invalid
        if !s.contains(",") {
            return None;
        }

        let row_col: Vec<&str> = s.split(",").map(|part| part.trim()).collect();
        // if has more than 1 - definitely invalid
        if row_col.len() != 2 {
            return None;
        }

        loc.row = row_col[0].parse::<usize>().unwrap_or(0);
        loc.col = row_col[1].parse::<usize>().unwrap_or(0);

        Some(loc)
    }

    fn is_on_board(&self, board_size: (usize, usize)) -> bool {
        self.row < board_size.0 - 1 && self.col < board_size.1 - 1
    }

    fn coords(&self, rows: usize) -> String {
        let col = "ABCDEFGHJKLMNOPQRST";
        let row = rows - self.row;
        let col = col.chars().nth(self.col).unwrap();
        return format!("{}{}", col, row);
    }

    // Checking borders for each "island"
    fn get_bordering_colors(&self, island: &Vec<Loc>, board: &Board) -> HashSet<Color> {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub player: Player,
    pub loc: Loc,
}

impl Move {
    pub fn pass(
        &self,
        black_pass: &mut bool,
        black_pass_cnt: &mut usize,
        white_pass: &mut bool,
        white_pass_cnt: &mut usize,
    ) {
        match self.player {
            Player::Black => {
                *black_pass = true;
                *black_pass_cnt += 1;
            }
            Player::White => {
                *white_pass = true;
                *white_pass_cnt += 1;
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    pub fields: Vec<Vec<Color>>,
    pub game_history: Vec<Move>,
    pub komi: usize,
}

impl Default for Board {
    fn default() -> Self {
        // Initializing an empty boardą
        let mut board = Board {
            fields: vec![vec![Color::Empty; 15]; 15],
            game_history: vec![],
            komi: 2,
        };
        // Setting up sentinels in rows
        for i in 0..15 {
            board.fields[0][i] = Color::Invalid;
            board.fields[14][i] = Color::Invalid;
        }
        // Setting up sentinels in columns
        for i in 0..15 {
            board.fields[i][0] = Color::Invalid;
            board.fields[i][14] = Color::Invalid;
        }
        board
    }
}

impl Board {
    pub fn new(rows: usize, cols: usize, komi: usize) -> Self {
        // Initializing an empty boardą
        let mut board = Board {
            fields: vec![vec![Color::Empty; cols]; rows],
            game_history: vec![],
            komi,
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

    fn set(&mut self, loc: Loc, color: Color) {
        self.fields[loc.row][loc.col] = color;
    }

    pub fn print_board(&self) {
        for row in &self.fields {
            for field in row {
                print!("{} ", field.to_string());
            }
            println!();
        }
    }

    fn board_size(&self) -> (usize, usize) {
        (self.fields.len(), self.fields[0].len())
    }

    fn count_stones(&self) -> (usize, usize) {
        let mut black: usize = 0;
        let mut white: usize = 0;
        for row in &self.fields {
            for field in row {
                if *field == Color::White {
                    white += 1;
                } else if *field == Color::Black {
                    black += 1;
                }
            }
        }
        (black, white)
    }

    pub fn calculate_captures(
        &self,
        black_pass_counter: usize,
        white_pass_counter: usize,
    ) -> (usize, usize) {
        let number_of_moves: usize =
            self.game_history.len() + black_pass_counter + white_pass_counter;
        let expected_black_stones: usize =
            number_of_moves / 2 + number_of_moves % 2 - black_pass_counter;
        let expected_white_stones: usize = number_of_moves / 2 - white_pass_counter;
        let (black_stones, white_stones) = self.count_stones();
        let black_captures = expected_white_stones - white_stones;
        let white_captures = expected_black_stones - black_stones;
        println!(
            "Black captured {:?} stones,\nwhite captured {:?} stones\n",
            black_captures, white_captures
        );
        (black_captures, white_captures)
    }

    fn get_all_loc(&self) -> Vec<Loc> {
        let mut all_loc: Vec<Loc> = vec![];
        for (i, row) in self.fields.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                all_loc.push(Loc { row: i, col: j })
            }
        }
        all_loc
    }
    // TODO: Improve performance by checking if this group has already been added
    // Creates a set of potential points - "islands" of Color::Empty
    fn create_set_of_potential_points(&self) -> HashSet<Vec<Loc>> {
        let mut groups_of_potential_points: HashSet<Vec<Loc>> = HashSet::new();
        let all_loc = self.get_all_loc();
        for loc in all_loc {
            if self.get(loc) == Color::Empty {
                groups_of_potential_points.insert(self.group_stones(loc));
            }
        }
        groups_of_potential_points
    }

    pub fn count_potential_points(&self, loc: Loc) -> (Color, usize) {
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

        let mut points: (Color, usize) = (Color::Empty, 0);

        if !potential_points_are_dame {
            if bordering_colors.contains(&Color::Black) {
                points = (Color::Black, group.len());
            }
            if bordering_colors.contains(&Color::White) {
                points = (Color::White, group.len());
            }
        }

        points
    }

    // Grouping empty "islands" and checking bordering Colors to decide which Color the points belong
    pub fn count_board_points(&self) -> (usize, usize) {
        // Populating the HashSet of Empty "islands"
        let groups_of_potential_points = self.create_set_of_potential_points();

        let mut white_points: usize = 0;
        let mut black_points: usize = 0;

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

    pub fn count_score(&self, board_points: (usize, usize), captures: (usize, usize), komi: usize) {
        let black_total_points: usize = board_points.0 + captures.0;
        let white_total_points: usize = board_points.1 + captures.1 + komi;
        let black_won = black_total_points > white_total_points;
        if black_won {
            println!(
                "Black won by: {}.5",
                black_total_points - white_total_points - 1
            );
        } else {
            println!(
                "White won by: {}.5",
                white_total_points - black_total_points
            );
        }
    }

    pub fn change_player(&self, mv: &mut Move) {
        mv.player = match mv.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
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
        let mut board_from_2_moves_ago =
            Board::new(self.fields.len(), self.fields[0].len(), self.komi);
        for mv in gh_copy {
            board_from_2_moves_ago.play(&mv);
        }
        // If the group has been removed after the move, it was a suicidcal move
        let move_is_suicidal = potential_board.get(mv.loc) == Color::Empty;
        let board_is_repeated = board_from_2_moves_ago.board_position_is_reapeated(potential_board);

        !move_is_suicidal && !board_is_repeated
    }

    pub fn play(&mut self, mv: &Move) {
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

pub fn sort(mut group: Vec<Loc>) -> Vec<Loc> {
    group.sort_by(|a, b| a.row.cmp(&b.row).then(a.col.cmp(&b.col)));
    group
}

pub fn take_player_input() -> String {
    let mut player_input = String::new();
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input");
    player_input = player_input.trim().to_string();
    player_input
}
