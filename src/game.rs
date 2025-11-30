use wasm_bindgen::JsValue;
use js_sys::Math;


// Embed editable word list (one word per line)
static WORD_LIST: &str = include_str!("../assets/EN_5-char_wordlist.txt");


#[derive(Clone)]
pub struct GameConfig {
    pub rows: usize,
    pub word_len: usize,
}


impl Default for GameConfig {
    fn default() -> Self {
        Self { rows: 6, word_len: 5 }
    }
}



#[derive(Clone)]
pub struct GameState {
    pub config: GameConfig,
    pub words: Vec<String>,
    pub target: String,
    pub board: Vec<Vec<char>>, // rows x word_len
    pub row: usize,
    pub col: usize,
}


impl GameState {
    pub fn new(config: GameConfig, words: Vec<String>, target: String) -> Self {
        let board = vec![vec![' '; config.word_len]; config.rows];
        Self { config, words, target, board, row: 0, col: 0 }
    }


    pub fn add_letter(&mut self, c: char) {
        if self.col < self.config.word_len && self.row < self.config.rows {
            self.board[self.row][self.col] = c;
            self.col += 1;
        }
    }


    pub fn del_letter(&mut self) {
        if self.col > 0 && self.row < self.config.rows {
            self.col -= 1;
            self.board[self.row][self.col] = ' ';
        }
    }


    pub fn submit_row(&mut self) -> Option<Vec<CellMark>> {
        if self.col != self.config.word_len || self.row >= self.config.rows {
            return None; // incomplete or no more rows
        }


        let guess: String = self.board[self.row].iter().collect();
        let target_chars: Vec<char> = self.target.chars().collect();


        // Basic Wordle marking: correct -> present -> absent (single-pass with counts)
        let mut marks = vec![CellMark::Absent; self.config.word_len];
        let mut target_count = std::collections::HashMap::new();


        for &tc in &target_chars {
            *target_count.entry(tc).or_insert(0usize) += 1;
        }


        // First pass: exact matches
        for i in 0..self.config.word_len {
            let g = self.board[self.row][i];
            if g == target_chars[i] {
                marks[i] = CellMark::Correct;
                *target_count.get_mut(&g).unwrap() -= 1;
            }
        }


        // Second pass: present but not already matched
        for i in 0..self.config.word_len {
            if marks[i] == CellMark::Correct { continue }
            let g = self.board[self.row][i];
            if let Some(&cnt) = target_count.get(&g) {
                if cnt > 0 {
                    marks[i] = CellMark::Present;
                    *target_count.get_mut(&g).unwrap() -= 1;
                }
            }
        }


        self.row += 1;
        self.col = 0;


        Some(marks)
    }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellMark { Correct, Present, Absent }


// Load words from the embedded text file
pub fn load_words() -> Vec<String> {
    WORD_LIST
        .lines()
        .map(|w| w.trim().to_uppercase())
        .filter(|w| !w.is_empty())
        .collect()
}


// Choose a random target using JS Math.random() (works in WASM)
pub fn choose_target(words: &[String], config: &GameConfig) -> String {
    let filtered: Vec<&String> = words.iter().filter(|w| w.len() == config.word_len).collect();
    if filtered.is_empty() {
        panic!("No words of the required length in assets/words.txt");
    }
    let idx = (Math::random() * (filtered.len() as f64)).floor() as usize;
    filtered[idx].clone()
}






