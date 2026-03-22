//! Naming screen state machine.
//!
//! Replicates `engine/menus/naming_screen.asm`:
//! - 3 screen types: Player name, Rival name, Pokémon nickname
//! - Player/Rival max 7 chars (PLAYER_NAME_LENGTH - 1)
//! - Pokémon nickname max 10 chars (NAME_LENGTH - 1)
//! - Alphabet grid: 5 rows × 9 columns, uppercase/lowercase toggle
//! - Special row 5: contains "ED" (submit) tile at col 8
//! - Row 6 (index 5 zero-based in grid): case toggle row
//! - Cursor navigates rows 0..=5 (grid rows 0-4 + case row)
//! - Columns 0..=8 in grid (mapped from wTopMenuItemX 1,3,5,...,17 → 0..8)
//!
//! The ASM uses wCurrentMenuItem (1-based rows 1-6, with 6=case row)
//! and wTopMenuItemX (odd values 1-17 for 9 columns).
//! We simplify to 0-based row/col.

/// Max name length for player/rival (excluding terminator).
pub const PLAYER_NAME_MAX: usize = 7;

/// Max name length for Pokémon nickname (excluding terminator).
pub const MON_NAME_MAX: usize = 10;

/// Alphabet grid dimensions.
pub const GRID_ROWS: usize = 5;
pub const GRID_COLS: usize = 9;

/// Total cursor rows: 5 grid rows + 1 case toggle row = 6.
pub const TOTAL_ROWS: usize = 6;

/// The uppercase alphabet grid (5 rows × 9 cols).
/// Row 4 (last data row) ends with the ED tile at col 8.
pub const UPPER_ALPHABET: [[char; GRID_COLS]; GRID_ROWS] = [
    ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'],
    ['J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R'],
    ['S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' '],
    ['×', '(', ')', ':', ';', '[', ']', 'é', 'ё'],
    ['-', '?', '!', '♂', '♀', '/', '.', ',', '⊡'],
];

/// The lowercase alphabet grid.
pub const LOWER_ALPHABET: [[char; GRID_COLS]; GRID_ROWS] = [
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
    ['j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r'],
    ['s', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' '],
    ['×', '(', ')', ':', ';', '[', ']', 'é', 'ё'],
    ['-', '?', '!', '♂', '♀', '/', '.', ',', '⊡'],
];

/// The ED tile marker character (row 4, col 8).
pub const ED_CHAR: char = '⊡';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingScreenType {
    Player,
    Rival,
    Pokemon,
}

impl NamingScreenType {
    pub fn max_length(&self) -> usize {
        match self {
            Self::Player | Self::Rival => PLAYER_NAME_MAX,
            Self::Pokemon => MON_NAME_MAX,
        }
    }
}

/// Extended input for the naming screen (needs all 8 buttons).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamingInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl NamingInput {
    pub fn none() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }
}

/// Result of a naming screen frame update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamingScreenResult {
    /// Still editing — no final action yet.
    Editing,
    /// User submitted a name (Start or selected ED tile).
    Submitted(String),
    /// User cancelled (submitted empty name, per ASM '@' check).
    Cancelled,
}

/// The naming screen state machine.
#[derive(Debug, Clone)]
pub struct NamingScreenState {
    screen_type: NamingScreenType,
    /// Current name buffer.
    name: String,
    /// Whether we're showing lowercase (true) or uppercase (false).
    lowercase: bool,
    /// Cursor row (0-based): 0-4 = alphabet grid rows, 5 = case toggle row.
    cursor_row: usize,
    /// Cursor column (0-based): 0-8.
    cursor_col: usize,
    /// Whether the name has been submitted.
    submitted: bool,
}

impl NamingScreenState {
    pub fn new(screen_type: NamingScreenType) -> Self {
        Self {
            screen_type,
            name: String::new(),
            lowercase: false,
            cursor_row: 0,
            cursor_col: 0,
            submitted: false,
        }
    }

    pub fn screen_type(&self) -> NamingScreenType {
        self.screen_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn max_length(&self) -> usize {
        self.screen_type.max_length()
    }

    pub fn is_lowercase(&self) -> bool {
        self.lowercase
    }

    pub fn cursor_row(&self) -> usize {
        self.cursor_row
    }

    pub fn cursor_col(&self) -> usize {
        self.cursor_col
    }

    /// Get the current alphabet grid based on case.
    pub fn current_alphabet(&self) -> &[[char; GRID_COLS]; GRID_ROWS] {
        if self.lowercase {
            &LOWER_ALPHABET
        } else {
            &UPPER_ALPHABET
        }
    }

    /// Process one frame of input. Returns the result.
    pub fn update_frame(&mut self, input: NamingInput) -> NamingScreenResult {
        if self.submitted {
            return self.submit_result();
        }

        // Start = submit immediately (per ASM .pressedStart)
        if input.start {
            self.submitted = true;
            return self.submit_result();
        }

        // Select = toggle case (per ASM .pressedSelect)
        if input.select {
            self.lowercase = !self.lowercase;
            return NamingScreenResult::Editing;
        }

        // B = delete last char (per ASM .pressedB)
        if input.b {
            self.name.pop();
            return NamingScreenResult::Editing;
        }

        // A = type character or activate special tile
        if input.a {
            return self.handle_a_press();
        }

        // D-pad navigation
        if input.down {
            self.move_down();
        } else if input.up {
            self.move_up();
        } else if input.right {
            self.move_right();
        } else if input.left {
            self.move_left();
        }

        NamingScreenResult::Editing
    }

    fn handle_a_press(&mut self) -> NamingScreenResult {
        // Row 5 (case toggle row)
        if self.cursor_row == 5 {
            // In ASM, row 6 col 1 = case toggle. Other positions on row 6 also toggle.
            self.lowercase = !self.lowercase;
            return NamingScreenResult::Editing;
        }

        // Check if on the ED tile (row 4, col 8)
        if self.cursor_row == 4 && self.cursor_col == 8 {
            self.submitted = true;
            return self.submit_result();
        }

        // Normal character — add to name if space remains
        let ch = self.current_alphabet()[self.cursor_row][self.cursor_col];
        if self.name.len() < self.max_length() {
            self.name.push(ch);

            // Per ASM: when all spaces filled, force cursor to ED tile
            if self.name.len() >= self.max_length() {
                self.cursor_row = 4;
                self.cursor_col = 8;
            }
        }

        NamingScreenResult::Editing
    }

    fn submit_result(&self) -> NamingScreenResult {
        if self.name.is_empty() {
            NamingScreenResult::Cancelled
        } else {
            NamingScreenResult::Submitted(self.name.clone())
        }
    }

    /// Move cursor down. Row wraps: 5 → 0 (per ASM .pressedDown, 6→1 in 1-based).
    fn move_down(&mut self) {
        self.cursor_row += 1;
        if self.cursor_row >= TOTAL_ROWS {
            self.cursor_row = 0;
        }
        // When entering case row (row 5), force col to 0 (per ASM behavior)
        if self.cursor_row == 5 {
            self.cursor_col = 0;
        }
    }

    /// Move cursor up. Row wraps: 0 → 5 (per ASM .pressedUp, 1→6 in 1-based).
    fn move_up(&mut self) {
        if self.cursor_row == 0 {
            self.cursor_row = 5;
            self.cursor_col = 0; // Force col on case row
        } else {
            self.cursor_row -= 1;
        }
    }

    /// Move cursor right. Wraps within row. Not available on case toggle row.
    fn move_right(&mut self) {
        if self.cursor_row == 5 {
            return; // Can't scroll on bottom row (per ASM)
        }
        self.cursor_col += 1;
        if self.cursor_col >= GRID_COLS {
            self.cursor_col = 0;
        }
    }

    /// Move cursor left. Wraps within row. Not available on case toggle row.
    fn move_left(&mut self) {
        if self.cursor_row == 5 {
            return; // Can't scroll on bottom row (per ASM)
        }
        if self.cursor_col == 0 {
            self.cursor_col = GRID_COLS - 1;
        } else {
            self.cursor_col -= 1;
        }
    }
}
