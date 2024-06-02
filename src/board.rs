// Lingo:
//     islands - sets of groups of Color::Empty from the Board

use std::collections::HashSet;
use std::{io, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
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
pub enum GameResult {
    Player(Player, f32),
    Draw,
}

impl GameResult {
    pub fn to_string(&self) -> String {
        match self {
            GameResult::Draw => String::from("D R A W !"),
            GameResult::Player(player, result) => match player {
                Player::Black => format!("Black +{}", result),
                Player::White => format!("White +{}", result),
            },
        }
    }
}

// TODO: move closer to the enum
impl Player {
    fn to_color(&self) -> Color {
        match self {
            Player::Black => Color::Black,
            Player::White => Color::White,
        }
    }

    pub fn opponent(self) -> Self {
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

    // TODO: return all 4 neighbours, use in loops
    //fn all_4nbr(self) -> Vec<Self> 
    
    pub fn from_string(s: &str) -> Option<Self> {
        if !s.contains(",") {
            return None;
        }

        let row_col: Vec<&str> = s.split(",").map(|part| part.trim()).collect();
        // if has more than 1 - definitely invalid
        if row_col.len() != 2 {
            return None;
        }

        // parse() returns Result<T, E>, ok() converts it to Option<T>
        let row = row_col[0].parse::<usize>().ok()?;
        let col = row_col[1].parse::<usize>().ok()?;

        Some(Loc { row, col })
    }

    fn is_on_board(&self, board_size: (usize, usize)) -> bool {
        let upper_edge_check = self.row > 0;
        let lower_edge_check = self.row < board_size.0 - 1;
        let left_edge_check = self.col > 0;
        let right_edge_check = self.col < board_size.1 - 1;

        upper_edge_check && lower_edge_check && left_edge_check && right_edge_check
    }

    // TODO: introduce struct BoardSize(usize, usize) and store it in board and pass here.
    fn get_all_on_board(r: usize, c: usize) -> Vec<Loc> {
        let mut all_loc: Vec<Loc> = vec![];
        // TODO: bug: you don't want to iterate over sentinels
        for row in 0..r {
            for col in 0..c {
                all_loc.push(Loc { row, col })
            }
        }
        all_loc
    }

    pub fn pass() -> Self {
        Loc { row: 99, col: 99 }
    }

    fn is_pass(&self) -> bool {
        self.row == 99 && self.col == 99
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
            loc: Loc { row: 99, col: 99 }, // TODO: if you happen to change the representation of pass to 98 98, this would be a bug. Please remember to reuse higher abstractions.
        }
    }

    pub fn is_pass(&self) -> bool {
        // self.loc == Loc::pass()
        match self.loc {
            Loc { row: 99, col: 99 } => true,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    fields: Vec<Vec<Color>>,
    game_history: Vec<Move>,
    current_player: Player,
    komi: f32,
    black_captures: isize,
    white_captures: isize,
}

impl Board {
    // TODO use BoardSize
    pub fn new(rows: usize, cols: usize, komi: f32) -> Self {
        // Initializing an empty board
        let mut board = Board {
            fields: vec![vec![Color::Empty; cols]; rows],
            game_history: vec![],
            current_player: Player::Black,
            komi,
            black_captures: 0,
            white_captures: 0,
        };
        // Setting up sentinels in rows
        // TODO a better algo would be to set sentinels everywhere and then iterate over Loc::all_on_board()
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

    fn reset(&self) -> Self {
        return Board::new(self.fields.len(), self.fields[0].len(), self.komi);
    }

    pub fn get_game_history(&self) -> &Vec<Move> {
        &self.game_history
    }

    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    pub fn set_current_player(&mut self, player: Player) {
        self.current_player = player;
    }

    fn get(&self, loc: Loc) -> Color {
        self.fields[loc.row][loc.col]
    }

    fn set(&mut self, loc: Loc, color: Color) {
        self.fields[loc.row][loc.col] = color;
    }

    pub fn to_string(&self) -> String {
        // TODO: add standard coordinates on sides.
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

    // Creates a set of potential points
    fn empty_islands(&self) -> HashSet<Vec<Loc>> {
        let mut islands: HashSet<Vec<Loc>> = HashSet::new();
        let (rows, cols) = self.board_size();
        for loc in Loc::get_all(rows, cols) {
            if self.get(loc) == Color::Empty {
                // If the group of Locs contains current Loc, the group of this loc has already been added
                if !islands.iter().any(|group| group.contains(&loc)) {
                    islands.insert(self.group_stones(loc));
                }
            }
        }
        islands
    }

    // Checking borders for each "island"
    fn get_bordering_colors(&self, island: &Vec<Loc>) -> HashSet<Color> {
        let mut bordering_colors: HashSet<Color> = HashSet::new();
        for field in island {
            bordering_colors.insert(self.get(field.up()));
            bordering_colors.insert(self.get(field.down()));
            bordering_colors.insert(self.get(field.left()));
            bordering_colors.insert(self.get(field.right()));
        }
        bordering_colors
    }

    fn count_potential_points(&self, loc: Loc) -> (Color, isize) {
        if self.get(loc) != Color::Empty {
            return (Color::Invalid, 0);
        }

        let group = self.group_stones(loc);
        let bordering_colors = self.get_bordering_colors(&group);

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
    fn count_board_points(&self) -> (isize, isize) {
        // Populating the HashSet of Empty "islands"
        let groups_of_potential_points = self.empty_islands();

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

    fn remove_dead_stones_for_counting(&mut self) {
        loop {
            println!("\nRemove dead stones or input 'r' to calculate the result:\n");
            println!("{}", self.to_string());

            let player_input = self::take_player_input();
            match player_input.as_str() {
                "r" => break,
                _ => match Loc::from_string(&player_input) {
                    None => {
                        println!("\nInvalid location :c\nInput one of the group's stone's location to remove it!");
                        continue;
                    }
                    Some(group_to_remove_loc) => self.remove_group(group_to_remove_loc),
                },
            }
        }
    }

    pub fn count_score(&mut self) -> GameResult {
        self.remove_dead_stones_for_counting();
        let all_points = self.count_board_points();
        let black_total_points: f32 = all_points.0 as f32 + self.black_captures as f32;
        let white_total_points: f32 = all_points.1 as f32 + self.white_captures as f32 + self.komi;

        if black_total_points - white_total_points == 0.0 {
            return GameResult::Draw;
        }

        let black_won = black_total_points > white_total_points;
        let result: GameResult;

        if black_won {
            result = GameResult::Player(Player::Black, black_total_points - white_total_points);
        } else {
            result = GameResult::Player(Player::White, white_total_points - black_total_points);
        }

        result
    }

    #[allow(dead_code)]
    fn board_position_is_reapated(&self, board: Board) -> bool {
        self.fields == board.fields
    }

    #[allow(dead_code)]
    fn move_is_valid(&self, mv: &Move) -> bool {
        let board_size = self.board_size();
        if !mv.loc.is_on_board(board_size) && !mv.loc.is_pass() {
            return false;
        }

        if self.get(mv.loc) != Color::Empty {
            return false;
        }

        let mut potential_board = self.clone();
        if potential_board.get(mv.loc) == Color::Empty {
            potential_board.unsafe_play(mv);
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
        let board_is_repeated = board_from_2_moves_ago.board_position_is_reapated(potential_board);

        !move_is_suicidal && !board_is_repeated
    }

    fn unsafe_play(&mut self, mv: &Move) {
        self.game_history.push(mv.clone());

        if mv.is_pass() {
            self.current_player = self.current_player.opponent();
            return;
        }

        self.set(mv.loc, mv.player.to_color());
        self.current_player = self.current_player.opponent();

        // Remove dead groups
        fn get_check_invalid_remove_group_combo(board: &mut Board, loc: Loc) {
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

    #[allow(dead_code)]
    // TODO return info whether there was a success and use it in main loop
    pub fn play(&mut self, mv: &Move) {
        if self.move_is_valid(mv) {
            self.unsafe_play(mv);
        }
    }

    fn group_stones(&self, loc: Loc) -> Vec<Loc> {
        let mut group_stones_coordinates: Vec<Loc> = vec![];
        let color = self.fields[loc.row][loc.col];
        self.flood_fill(loc, color, &mut group_stones_coordinates);
        group_stones_coordinates.sort();
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
        if self.game_history.len() == 0 {
            return self;
        }

        let mut board_after_undo = self.reset();
        self.game_history.pop();
        for mv in &self.game_history {
            board_after_undo.play(mv);
        }
        board_after_undo
    }
    // When the argument is (self), not (&self), cloning the board will be needed at every iteration of the while loop
    pub fn last_two_moves_are_pass(&self) -> bool {
        if self.game_history.len() > 1 {
            let last_two_moves = &self.game_history[self.game_history.len() - 2..];
            return last_two_moves[0].loc == last_two_moves[1].loc; 
            // TODO this is close to incorrect and very unreadable
            // 
        }
        false
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
        let mut board = Board::new(5, 5, 0.0);
        assert_eq!(board.get(Loc { row: 1, col: 1 }), Color::Empty);
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        });

        assert_eq!(board.get(Loc { row: 1, col: 1 }), Color::Black);
        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 1, col: 1 },
        });

        assert_eq!(board.get(Loc { row: 1, col: 1 }), Color::Black);
        assert_eq!(board.get(Loc { row: 1, col: 2 }), Color::Empty);
        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 1, col: 2 },
        });
        assert_eq!(board.get(Loc { row: 1, col: 1 }), Color::Black);
        assert_eq!(board.get(Loc { row: 1, col: 2 }), Color::White);
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 2 },
        });
        assert_eq!(board.get(Loc { row: 1, col: 1 }), Color::Black);
        assert_eq!(board.get(Loc { row: 1, col: 2 }), Color::White);
    }

    #[test]
    fn stones_are_grouped_correctly() {
        let mut board = Board::new(11, 11, 2.0);

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
        let mut board = Board::new(11, 11, 2.0);

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
        let mut board = Board::new(11, 11, 2.0);

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

        board.remove_group(Loc { row: 1, col: 1 });
        assert!(board.get(Loc { row: 1, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 1, col: 2 }) == Color::Empty);
        board.remove_group(Loc { row: 5, col: 1 });
        assert!(board.get(Loc { row: 4, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 5, col: 1 }) == Color::Empty);
        board.remove_group(Loc { row: 3, col: 4 });
        assert!(board.get(Loc { row: 3, col: 3 }) == Color::Empty);
        assert!(board.get(Loc { row: 3, col: 4 }) == Color::Empty);
        assert!(board.get(Loc { row: 4, col: 3 }) == Color::Empty);
        board.remove_group(Loc { row: 6, col: 7 });
        assert!(board.get(Loc { row: 4, col: 7 }) == Color::Empty);
        assert!(board.get(Loc { row: 5, col: 7 }) == Color::Empty);
        assert!(board.get(Loc { row: 6, col: 7 }) == Color::Empty);
        board.remove_group(Loc { row: 3, col: 2 });
        assert!(board.get(Loc { row: 2, col: 2 }) == Color::Empty);
        assert!(board.get(Loc { row: 3, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 3, col: 2 }) == Color::Empty);
        assert!(board.get(Loc { row: 4, col: 2 }) == Color::Empty);
        board.remove_group(Loc { row: 9, col: 1 });
        assert!(board.get(Loc { row: 9, col: 1 }) == Color::Empty);
        board.remove_group(Loc { row: 7, col: 3 });
        assert!(board.get(Loc { row: 6, col: 2 }) == Color::Empty);
        assert!(board.get(Loc { row: 6, col: 3 }) == Color::Empty);
        assert!(board.get(Loc { row: 7, col: 2 }) == Color::Empty);
        assert!(board.get(Loc { row: 3, col: 3 }) == Color::Empty);
        assert!(board.get(Loc { row: 8, col: 2 }) == Color::Empty);
    }

    #[test]
    fn groups_removal_is_triggered_when_their_liberties_reach_0() {
        let mut board = Board::new(11, 11, 2.0);

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
                board.play(&Move {
                    player: Player::Black,
                    loc: *mv,
                });
            }
        }

        for (group_index, white_moves) in white_groups.iter().enumerate() {
            for (i, mv) in white_moves.iter().enumerate() {
                board.play(&Move {
                    player: Player::White,
                    loc: *mv,
                });
                if i + 1 == white_moves.len() {
                    for loc in &black_groups[group_index] {
                        assert!(board.get(*loc) == Color::Empty);
                    }
                } else {
                    for loc in &black_groups[group_index] {
                        assert!(board.get(*loc) == Color::Black);
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

        let mut board = Board::new(7, 7, 2.0);
        let mut moves_left = 10;

        while moves_left > 0 {
            let row = rng.gen_range(0..7);
            let col = rng.gen_range(0..7);
            let current_move_coords = Loc { row, col };
            current_move.loc = current_move_coords;

            if board.move_is_valid(&current_move) {
                test_move_history.push(current_move.clone());
                board.play(&current_move);
                current_move.player = current_move.player.opponent();
                moves_left -= 1;
            }
        }

        for _ in 1..=6 {
            let last_move = test_move_history.pop().unwrap();
            assert_ne!(board.get(last_move.loc), Color::Empty);

            board = board.undo();

            assert_eq!(board.get(last_move.loc), Color::Empty);
        }

        moves_left = 6;

        while moves_left > 0 {
            let row = rng.gen_range(0..7);
            let col = rng.gen_range(0..7);
            let current_move_coords = Loc { row, col };
            current_move.loc = current_move_coords;

            if board.move_is_valid(&current_move) {
                assert_eq!(board.get(current_move.loc), Color::Empty);
                board.play(&current_move);
                assert_ne!(board.get(current_move.loc), Color::Empty);
                current_move.player = current_move.player.opponent();

                println!();
                moves_left -= 1;
            }
        }
    }

    #[test]
    fn undo_restores_both_groups_that_were_captured_by_the_undone_move() {
        let mut board = Board::new(7, 5, 2.0);

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
            board.play(&mv);
        }

        board = board.undo();

        assert!(board.get(Loc { row: 1, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 2, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 3, col: 1 }) == Color::White);
    }

    #[test]
    fn board_position_cannot_be_repeated() {
        let mut board = Board::new(6, 5, 2.0);

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
            board.play(&mv);
        }

        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 2, col: 1 },
        });

        assert!(board.get(Loc { row: 2, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 1, col: 1 }) == Color::Black);

        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 4, col: 3 },
        });
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 3, col: 3 },
        });
        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 2, col: 1 },
        });
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        });

        assert!(board.get(Loc { row: 1, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 2, col: 1 }) == Color::White);

        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 2, col: 3 },
        });
        board.play(&Move {
            player: Player::White,
            loc: Loc { row: 4, col: 2 },
        });
        board.play(&Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        });

        assert!(board.get(Loc { row: 2, col: 1 }) == Color::Empty);
        assert!(board.get(Loc { row: 1, col: 1 }) == Color::Black);
    }

    #[test]
    fn each_group_points_are_counted_correctly() {
        let mut board = Board::new(8, 8, 0.0);
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
        let mut board = Board::new(8, 8, 0.0);
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

        assert_eq!(board.count_board_points(), (3, 8));
    }

    #[test]
    fn passing_works() {
        let mut current_move = Move {
            player: Player::Black,
            loc: Loc { row: 1, col: 1 },
        };
        let expected_move = current_move.clone();
        assert_eq!(current_move, expected_move);

        current_move = current_move.pass();
        assert!(current_move.is_pass());

        current_move = Move {
            player: Player::White,
            loc: Loc { row: 5, col: 3 },
        };

        current_move = current_move.pass();
        assert!(current_move.is_pass());
    }
    #[test]
    fn counting_captures() {
        let mut board = Board::new(8, 8, 0.0);

        let black_groups = [
            // Capture 1
            Loc { row: 1, col: 1 },
            // Capture 2
            Loc { row: 1, col: 5 },
            Loc { row: 2, col: 6 },
            // Capture 3
            Loc { row: 4, col: 1 },
            Loc { row: 5, col: 1 },
            Loc { row: 5, col: 3 },
            Loc { row: 6, col: 2 },
            Loc { row: 6, col: 3 },
        ];

        let white_capture_1 = [Loc { row: 1, col: 2 }, Loc { row: 2, col: 1 }];

        let white_capture_2 = [
            Loc { row: 1, col: 4 },
            Loc { row: 2, col: 5 },
            Loc { row: 3, col: 6 },
            Loc { row: 1, col: 6 },
        ];

        let white_capture_3 = [
            Loc { row: 3, col: 1 },
            Loc { row: 4, col: 2 },
            Loc { row: 4, col: 3 },
            Loc { row: 5, col: 2 },
            Loc { row: 5, col: 4 },
            Loc { row: 6, col: 4 },
            Loc { row: 6, col: 1 },
        ];

        for loc in black_groups {
            board.play(&Move {
                player: Player::Black,
                loc,
            });
        }

        assert_eq!(board.white_captures, 0);
        assert_eq!(board.black_captures, 0);

        for loc in white_capture_1 {
            board.play(&Move {
                player: Player::White,
                loc,
            });
        }

        assert_eq!(board.white_captures, 1);
        assert_eq!(board.black_captures, 0);

        for loc in white_capture_2 {
            board.play(&Move {
                player: Player::White,
                loc,
            });
        }

        assert_eq!(board.white_captures, 3);
        assert_eq!(board.black_captures, 0);

        for loc in white_capture_3 {
            board.play(&Move {
                player: Player::White,
                loc,
            });
        }

        assert_eq!(board.white_captures, 8);
        assert_eq!(board.black_captures, 0);
    }
}
