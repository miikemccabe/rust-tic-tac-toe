use std::fmt;

#[derive(Debug)]
pub struct Board {
  grid: [[CellValue; 3]; 3]
}

impl Board {
  pub fn new() -> Self {
      Board {
          grid: [
              [CellValue::Empty, CellValue::Empty, CellValue::Empty],
              [CellValue::Empty, CellValue::Empty, CellValue::Empty],
              [CellValue::Empty, CellValue::Empty, CellValue::Empty],
          ]
      }
  }

  pub fn clear(&mut self) {
      self.grid = [
          [CellValue::Empty, CellValue::Empty, CellValue::Empty],
          [CellValue::Empty, CellValue::Empty, CellValue::Empty],
          [CellValue::Empty, CellValue::Empty, CellValue::Empty],
      ]
  }

  pub fn display(&self) {
      println!("     1           2          3     ");
      println!("+----------+----------+----------+");
      self.print_row(0);
      println!("+----------+----------+----------+");
      self.print_row(1);
      println!("+----------+----------+----------+");
      self.print_row(2);
      println!("+----------+----------+----------+");
  }

  pub fn set_cell(&mut self, row: usize, col: usize, value: Player) -> Result<(), &str> {
      if !self.cell_in_range(&row, &col) {
        return Err("Cell is out of range");
      }

      let mut success = false;
      let cell = self.get_cell(row, col);

      self.grid[row][col] = match cell {
          None => {
              success = true;
              CellValue::Player(value)
          },
          Some(player) => CellValue::Player(*player)
      };

      match success {
          true => Ok(()),
          false => Err("Cell is already populated")
      }
  }

  fn cell_in_range(&self, &row: &usize, &col: &usize) -> bool {
    (0..3).contains(&row) & (0..3).contains(&col)
  }

  pub fn get_cell(&self, row: usize, col: usize) -> Option<&Player> {
      if !self.cell_in_range(&row, &col) {
        return None;
      }

      match &self.grid[row][col] {
          CellValue::Player(player) => Some(player),
          CellValue::Empty => None
      }
  }

  fn print_row(&self, row: usize) {
      let row_label = match row {
          0 => String::from("A"),
          1 => String::from("B"),
          2 => String::from("C"),
          _ => String::from("")
      };
      println!("|          |          |          |");
      println!("|     {}    |     {}    |     {}    |   {}", self.grid[row][0], self.grid[row][1], self.grid[row][2], row_label);
      println!("|          |          |          |");
  }
}

#[derive(Debug)]
enum CellValue {
  Player(Player),
  Empty
}

impl std::fmt::Display for CellValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self {
          CellValue::Player(player) => player.fmt(f),
          CellValue::Empty => write!(f, " ")
      }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    Naught,
    Cross
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Player::Naught => write!(f, "○"),
            Player::Cross => write!(f, "⨯")
        }
    }
}
