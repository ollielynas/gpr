
use std::{sync::Mutex, time::Duration};

use game::Game;
use wasm_bindgen::prelude::*;
use wasm_cookies::CookieOptions;

mod game;
mod tic_tac_toe;

pub static GAME_STATE: Mutex<Game> = Mutex::new(Game::StaticError);


// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn accept_move(action: String, game_id: String) {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.clone().dyn_into::<web_sys::HtmlDocument>().unwrap();
    let mut cookie = html_document.cookie().unwrap();

    *GAME_STATE.lock().unwrap() = match wasm_cookies::cookies::get(&cookie, &game_id) {
    Some(Ok(s)) => Game::from_string(s),
    Some(Err(s)) => Game::Error(s.to_string()),
    None => Game::new_from_action(action.clone()),
    };


    (*GAME_STATE.lock().unwrap()).accept_move(action);

    document.get_element_by_id("game-body").expect("game body doesn't exist").set_inner_html(&GAME_STATE.lock().unwrap().get_html());

    cookie = wasm_cookies::cookies::set(&game_id, &*&GAME_STATE.lock().unwrap().to_string(), &CookieOptions::default().expires_after(Duration::from_secs(6048000)));

    html_document.set_cookie(&cookie);


    // html_document.set_cookie()
}


#[wasm_bindgen]
pub fn make_move(action: String, game_id: String) {
    
}



