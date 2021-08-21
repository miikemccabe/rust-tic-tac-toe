use crate::board::Board;
use crate::board::Player;

pub struct Game {
    pub player: Player,
    pub board: Board,
}

impl Game {

    pub fn new() -> Self {
        Game { 
            board: Board::new(),
            player: Player::Cross
        }
    }

    pub fn display(&self) {
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        self.board.display();
    }

    pub fn reset_board(&mut self) {
        self.board.clear();
        self.player = Player::Cross;
    }

    pub fn play_turn(&mut self, row: usize, col: usize) {
        match self.board.set_cell(row, col, self.player) {
            Ok(()) => self.toggle_player(),
            Err(err) => println!("{}", err)
        }
    }

    fn toggle_player(&mut self) {
        self.player = match self.player {
            Player::Cross => Player::Naught,
            Player::Naught => Player::Cross
        }
    }
}
