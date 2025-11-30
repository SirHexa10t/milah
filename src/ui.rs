use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, KeyboardEvent};
use std::rc::Rc;
use std::cell::RefCell;
use crate::game::{GameState, CellMark};


pub struct Ui {
    document: Document,
}


impl Ui {
    pub fn new() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        Ok(Self{ document })
    }


    pub fn build_grid(&self, state: &GameState) -> Result<(), JsValue> {
        let root = self.document.get_element_by_id("app").ok_or_else(|| JsValue::from_str("#app not found"))?;
        root.set_inner_html("");


        for r in 0..state.config.rows {
            let row_el = self.document.create_element("div")?;
            row_el.set_class_name("row");


            for c in 0..state.config.word_len {
                let cell = self.document.create_element("div")?;
                cell.set_class_name("cell");
                let id = format!("cell-{}-{}", r, c);
                cell.set_attribute("id", &id)?;
                row_el.append_child(&cell)?;
            }


            root.append_child(&row_el)?;
        }


        Ok(())
    }



    /// Install event listeners. Accept a shared, mutable GameState so the closure can borrow it.
    pub fn install_event_listeners(&self, shared_state: Rc<RefCell<GameState>>) -> Result<(), JsValue> {
        let window = window().unwrap();
        let document = self.document.clone();


        // Keydown listener attached to `window` (works well across focus changes)
        let sh = shared_state.clone();
        let doc_for_ui = document.clone();
        let keydown_closure = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let key = ev.key();
            let mut st = sh.borrow_mut();


            if key.len() == 1 && key.chars().all(|c| c.is_ascii_alphabetic()) {
                let ch = key.to_ascii_uppercase().chars().next().unwrap();
                st.add_letter(ch);
                // update the last filled cell (col - 1)
                let row = st.row;
                let col = st.col.saturating_sub(1);
                Ui::update_cell_ui(&doc_for_ui, row, col, ch, "");
            } else if key == "Backspace" {
                st.del_letter();
                // update the cell that was cleared
                let row = st.row;
                let col = st.col;
                Ui::update_cell_ui(&doc_for_ui, row, col, ' ', "");
            } else if key == "Enter" {
                // Validate word exists in dictionary before accepting
                if st.col == st.config.word_len {
                    let guess: String = st.board[st.row].iter().collect();
                    if !st.words.contains(&guess) {
                        // simple feedback: flash row or log - here we console.warn
                        web_sys::console::log_1(&JsValue::from_str("Word not in dictionary"));
                        return;
                    }
                }



                if let Some(marks) = st.submit_row() {
                    for c in 0..st.config.word_len {
                        let ch = st.board[st.row.saturating_sub(1)][c];
                        let class = match marks[c] {
                            CellMark::Correct => "correct",
                            CellMark::Present => "present",
                            CellMark::Absent => "absent",
                        };
                        Ui::update_cell_ui(&doc_for_ui, st.row.saturating_sub(1), c, ch, class);
                    }
                }
            }


            // prevent unwanted browser side-effects for certain keys
            // (don't prevent for everything to keep browser accessibility intact)
            if key == "Enter" || key == "Backspace" {
                ev.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);


        // Attach and intentionally leak the closure so it lives for the lifetime of the page.
        window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
        keydown_closure.forget();


        Ok(())
    }


    fn update_cell_ui(document: &Document, row: usize, col: usize, ch: char, class: &str) {
        if let Some(el) = document.get_element_by_id(&format!("cell-{}-{}", row, col)) {
            el.set_inner_html(&ch.to_string());
            if !class.is_empty() {
                el.set_attribute("class", &format!("cell {}", class)).ok();
            } else {
                el.set_attribute("class", "cell").ok();
            }
        }
    }
}
