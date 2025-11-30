mod game;
mod ui;


use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use crate::game::GameState;
use crate::ui::Ui;


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();


    let config = game::GameConfig::default();
    let words = game::load_words();
    let target = game::choose_target(&words, &config);


    let mut state = GameState::new(config, words, target);


    // Build UI
    let ui = Ui::new()?;
    ui.build_grid(&state)?;


    // Hook keyboard events (Ui takes care of wiring to state via closures)
    use std::rc::Rc;
    use std::cell::RefCell;
    let shared = Rc::new(RefCell::new(state));
    ui.install_event_listeners(shared.clone())?;


    Ok(())
}