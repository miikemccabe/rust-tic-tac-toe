use std::io;

pub enum Command {
  Clear,
  Quit
}

pub enum Choice {
  Cell((usize, usize)),
  Command(Command)
}

struct CoordsError(String);

pub fn get_choice() -> Result<Choice, String> {
  
  let mut cell = String::new();

  io::stdin()
      .read_line(&mut cell)
      .expect("Failed to read line");

  let choice = cell.trim();

  let command = parse_command(choice);
  let coords = parse_coords(choice);

  match command {
      Ok(command) => Ok(Choice::Command(command)),
      Err(()) => match coords {
          Ok(coords) => Ok(Choice::Cell(coords)),
          Err(CoordsError(message)) => {
              println!("{}", message);
              Err("Command not recognised".to_string())
          }
      }
  }
}

fn parse_command(command: &str) -> Result<Command, ()> {
  match command {
      "quit" => Ok(Command::Quit),
      "q" => Ok(Command::Quit),
      "clear" => Ok(Command::Clear),
      "c" => Ok(Command::Clear),
      _ => Err(())
  }
}

fn parse_coords(cell: &str) -> Result<(usize, usize), CoordsError> {

  if cell.len() > 2 {
     return Err(CoordsError(String::from("Too many coordinates")))
  }

  if cell.len() < 2 {
     return Err(CoordsError(String::from("Too few coordinates")))
  }

  // The user should enter a two digit coordinate for the cell like A2 or 3B. Splitting on "" produces an empty
  // string on either end of the array so we filter these about before collecting them as a Vector of &str.
  let mut cell = cell.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

  // Sort the array so that the user can enter the row or column in either order
  cell.sort_unstable();

  let column = cell[0].parse::<usize>();
  
  if column.is_err() {
      return Err(CoordsError(String::from("Can't parse column")));
  }

  let row: Option<usize> =  match cell[1].to_lowercase().as_ref() {
      "a" => Some(0),
      "b" => Some(1),
      "c" => Some(2),
      _ => None
  };
  
  if row.is_none() {
      return Err(CoordsError(String::from("Can't parse row")));
  }

  Ok((row.unwrap(), column.unwrap() - 1))
}
