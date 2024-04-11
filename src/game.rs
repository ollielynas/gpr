use std::{default, fmt::Error, usize};

use web_sys::window;

use crate::tic_tac_toe::TicTacToeState;






pub enum Game {
    // todo, make error value a generic that accepts anything that implements debug
    Error(String),
    StaticError,
    TicTacToe {rows: [TicTacToeState; 9], current_player: TicTacToeState},
}

impl Default for Game {
    fn default() -> Self {
        Game::Error("Defalt".to_string())
    }
}


// id format: [game type char][random number]
// cookie format: [id]=[game state]; expires=[date],



impl Game {

    pub fn winner(&self) -> i32 {
        return match self {
            Game::Error(_) => 0,
            Game::StaticError => 0,
            Game::TicTacToe { rows, current_player } => {
                let wins = vec![[0,1,2],[3,4,5],[6,7,8], [0,3,6], [1,4,7], [2,5,8], [0,4,8],[2,4,6]];
                for w in wins {
                    if [rows[w[0]],rows[w[1]],rows[w[2]]] == [TicTacToeState::O, TicTacToeState::O, TicTacToeState::O] {
                        return 1
                    }
                    if [rows[w[0]],rows[w[1]],rows[w[2]]] == [TicTacToeState::X, TicTacToeState::X, TicTacToeState::X] {
                        return 2
                    }
                }
                return 0;
            },
        }
    }

    /// this is run after processing the move made my your opponent
    pub fn get_html(&self, id: String, move_num: i32)  -> String {
        match self {
            Game::Error(e) =>format!("<p>error</p><p>{e}</p>"),
            Game::StaticError => "StaticError, game state was never loaded".to_owned(),
            Game::TicTacToe { rows, current_player } => {
                format!("{}",
                rows.to_vec().iter().enumerate().map(|x| match x {
                    (9..=usize::MAX, _) => String::new(),
                    (_ , TicTacToeState::O) => "<div class='grid3x3 tic-tac-toe'><p>O</p></div>".to_owned(),
                    (_ , TicTacToeState::X) => "<div class='grid3x3 tic-tac-toe'><p>X</p></div>".to_owned(),
                    (i , TicTacToeState::Blank) => {
                        let onclick = format!("make_move('T{}', '{}', '{}')", i, id, move_num + 1);
                        format!("<button class='grid3x3 tic-tac-toe' onclick = \"{onclick}\"></button>")
                    }
                }).collect::<String>() + "<div class='inner-outline'>"
            )
            },
        }
    }

    


    /// this is used to set the value of the global mut using the cookie data
    pub fn from_string(mut s:String) -> Game {
        if s.len() == 0 {
            return Game::Error("string passed in to 'from_string' was empty".to_owned());
        }
        let first = s.remove(0);


        return match first {
            'T' => { //                  ---    TIC TAC TOE  ----------
                const ARRAY_REPEAT_VALUE: TicTacToeState = TicTacToeState::Blank;
                let mut array = [ARRAY_REPEAT_VALUE;9];
                let last = match s.pop() {
                    Some(a) => a,
                    None => return Game::Error("game state has no length".to_owned()),
                };


                for (i, char) in s.chars().enumerate() {
                    if i >= 9 {break};
                    array[i] = match char {
                        'x'|'X' => TicTacToeState::X,
                        'o'|'O' => TicTacToeState::O,
                        '_' => TicTacToeState::Blank,
                        a => {
                            return Game::Error(format!("invalid tic tac toe state '{a}' in state '{s}'"))
                        }
                    }
                }
                Game::TicTacToe { rows: array, current_player: match last {
                    'x'|'X' => TicTacToeState::X,'o'|'O' => TicTacToeState::O, 
                    a => {
                        return Game::Error(format!("invalid state for next player '{a}' in state '{s}'"))
                    }
                } }
            }
            'E' => { //                       ---  ERROR -----
                Game::Error(s)
            }
            a => {//                    ----- OTHER ----
                Game::Error(format!("invalid game identifier: '{a}', game state: '{s}'"))
            }
        }
    }

    /// this is what the url parameters are passed into 
    pub fn accept_move(&mut self, mut action: String, action_number: i32, id: String) {
        let first = action.remove(0);

        if action == "" {
            return;
        }

        match self {
            Game::Error(ref mut a) => {*a = action},
            Game::StaticError => {},
            Game::TicTacToe {ref mut rows, ref mut current_player } => {
                let index = action.parse::<usize>().expect("i really need to make this into an error game");
                *current_player = match action_number%2 {
                    0 => TicTacToeState::O,
                    1 => TicTacToeState::X,
                    _ => unreachable!("wtf math has ceased to function")
                };

                if rows[index] == TicTacToeState::Blank {
                    rows[index] = *current_player;
                }
                

                *current_player = match action_number%2 {
                    0 => TicTacToeState::O,
                    1 => TicTacToeState::X,
                    _ => unreachable!("wtf math has ceased to function")
                };

            },
        }


        let game_body = window().unwrap().document().unwrap().get_element_by_id("game-body").unwrap();
        game_body.set_inner_html(&self.get_html(id, action_number));

    }

    /// this is what is saved as a cookie and then loaded when receiving a move
    pub fn to_string(&self) -> String {
        match self {
            Game::Error(e) => format!("E{e}"),
            Game::TicTacToe { rows , current_player} => {
                format!("T{}{}", rows.map(|x| x.to_char().to_string()).concat(), current_player.to_char())
            },
            Game::StaticError => "Estatic error".to_owned(),
        }
    }


    /// this is run if the game id is not found. 
    /// it is assumed that this is because it is the first move that they are making. i
    /// t is also assumed that they did not start the game
    pub fn new_from_action(mut action: String) -> Game {
        let c = action.remove(0);
        
        match c {
            'T' => {
                const ARRAY_REPEAT_VALUE: TicTacToeState = TicTacToeState::Blank;
                let mut array = [ARRAY_REPEAT_VALUE;9];
                Game::TicTacToe { rows: array, current_player: TicTacToeState::O }
            }
            'E' => Game::Error(format!("the move you were sent was for teh game type: Error. oops action: {action}")),
            a => Game::Error(format!("the move you were sent had an invalid game type: '{a}': Error. oops action: {action}")),

        }
    }

}