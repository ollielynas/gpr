
use std::{sync::Mutex, time::Duration};

use game::Game;
use wasm_bindgen::prelude::*;
use wasm_cookies::CookieOptions;
use fastrand;
use web_sys::{js_sys::Math::random, window};

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
pub fn accept_move(action: String, mut game_id: String, action_number: String) {
    fastrand::seed((random()*1000.0) as u64);
    if game_id.contains("RANDOM") {
        game_id = fastrand::choose_multiple("qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnm".chars(), 5).iter().map(|x| x).collect::<String>();
    }




    let action_number = match action_number.parse::<i32>() {
    Ok(a) => a,
    Err(a) => {
        *GAME_STATE.lock().unwrap() = Game::Error(format!("invalid move number: {a}"));
        return;
    },
    };



    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // panic!("test passed") ;
        *GAME_STATE.lock().unwrap() = match wasm_cookies::get(&game_id.clone()) {
    Some(Ok(s)) => Game::from_string(s),
    Some(Err(s)) => Game::Error(s.to_string()),
    None => Game::new_from_action(action.clone()),
    };
    }

    if action != "noaction".to_owned() {
    (*GAME_STATE.lock().unwrap()).accept_move(action.clone(), action_number, game_id.clone());
    }

    document.get_element_by_id("game-body").expect("game body doesn't exist").set_inner_html(&GAME_STATE.lock().unwrap().get_html(game_id.clone(), action_number));
    let game_type = action.clone().chars().next().unwrap();
    document.get_element_by_id("game-body").expect("game body doesn't exist").set_attribute("game-type", &game_type.to_string());


    #[cfg(target_arch = "wasm32")]
    wasm_cookies::set(&game_id.clone(), &*&GAME_STATE.lock().unwrap().to_string(), &CookieOptions::default().expires_after(Duration::from_secs(6048000)));


    // html_document.set_cookie()
}


#[wasm_bindgen]
pub fn make_move(action: String, game_id: String, action_number: i32) {


    let game_type = action.clone().chars().next().unwrap();
    (*GAME_STATE.lock().unwrap()).accept_move(action.clone(), action_number, game_id.clone());


    let winner = (*GAME_STATE.lock().unwrap()).winner();

    // must match dict in game.py

    let name = match game_type {
        'T' => "tic_tac_toe",
        _ => "error",
    };
    
    #[cfg(target_arch = "wasm32")]
    wasm_cookies::set(&game_id.clone(), &*&GAME_STATE.lock().unwrap().to_string(), &CookieOptions::default().expires_after(Duration::from_secs(6048000)));


    let params = format!("?id={game_id}&a={action}&num={action_number}");
    window().unwrap().location().set_href(&format!("https://ollielynas.github.io/gpr/close.html{params}&name={name}&w={winner}")).unwrap();
    // window().op
}

