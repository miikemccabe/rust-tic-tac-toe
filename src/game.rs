use crate::board::Board;
use crate::board::Player;

const WINNING_COMBOS: [[(usize, usize); 3]; 8]  = [
  [(0,0), (0,1), (0,2)],
  [(1,0), (1,1), (1,2)],
  [(2,0), (2,1), (2,2)],
  [(0,0), (1,0), (2,0)],
  [(0,1), (1,1), (2,1)],
  [(0,2), (1,2), (2,2)],
  [(0,0), (1,1), (2,2)],
  [(2,0), (1,1), (0,2)],
];

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

    pub fn find_winner(&self) -> Option<&Player> {
      let mut winner: Option<&Player> = None;

      for combo in WINNING_COMBOS {
        // Get the player at the first cell of the first winning combo
        let player = self.board.get_cell(combo[0].0, combo[0].1);
        for cell in combo {
          // Get the value of each cell in the potential winning combo
          let current = self.board.get_cell(cell.0, cell.1);
          // If this cell matches the current player, we've got a potential winner
          if current.is_some() && player.is_some() && current.unwrap() == player.unwrap() {
            // Tentatively set this as the winner and continue checking the rest of the combo
            winner = current;
          } else {
            // If any of the cells in this combo don't match the player this is not a winning combo. Give up on it.
            winner = None;
            break;
          }
        }
        if winner.is_some() {
          // If we have a winner after checking every cell in a combo, then we have a winner!
          break;
        }
      }

      winner
    }
}
