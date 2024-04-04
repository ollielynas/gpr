

#[derive(Clone, Copy, Debug)]
pub enum TicTacToeState {
    Blank,
    X,
    O,
}


impl TicTacToeState {
    pub fn to_char(&self) -> char {
        match self {
            TicTacToeState::Blank => '_',
            TicTacToeState::X => 'x',
            TicTacToeState::O => 'o',
        }
    }
}