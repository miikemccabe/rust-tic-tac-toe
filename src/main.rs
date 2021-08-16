use std::fmt;
use std::io;
// use rand::Rng;

struct Row<'a> {
    position: RowPosition,
    values: &'a [Value; 3],
}

#[derive(Clone, Copy)]
enum Value {
    Naught,
    Cross,
    None
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Naught => write!(f, "○"),
            Value::Cross => write!(f, "⨯"),
            Value::None => write!(f, " "),
        }
    }
}

enum RowPosition {
    Top,
    Middle,
    Bottom,
}

fn print_row_border() {
    println!("+----------+----------+----------+");
}

fn print_row_middle(values: &[Value; 3]) {
    println!("|          |          |          |");
    println!("|     {}    |     {}    |     {}    |", values[0], values[1], values[2]);
    println!("|          |          |          |");
}

fn print_row(row: Row) {
    match row.position {
        RowPosition::Top => { 
            print_row_border();
            print_row_middle(row.values);
        },
        RowPosition::Middle => {
            print_row_border();
            print_row_middle(row.values);
            print_row_border();
        },
        RowPosition::Bottom => {
            print_row_middle(row.values);
            print_row_border();
        }
    }
}

fn print_grid(grid: &mut[[Value; 3]; 3]) {
    let top_row = Row {
        position: RowPosition::Top,
        values: &grid[0],
    };
    let middle_row = Row {
        position: RowPosition::Middle,
        values: &grid[1],
    };
    let bottom_row = Row {
        position: RowPosition::Bottom,
        values: &grid[2],
    };
    print_row(top_row);
    print_row(middle_row);
    print_row(bottom_row);
}

fn main() {

    let mut grid: [[Value; 3]; 3] = [
        [Value::None, Value::None, Value::None],
        [Value::None, Value::None, Value::None],
        [Value::None, Value::None, Value::None],
    ];

    let mut player: Value = Value::Cross;

    print_grid(&mut grid);

    loop {

        println!("Player {}, choose your cell", player);

        let mut cell = String::new();

        io::stdin()
            .read_line(&mut cell)
            .expect("Failed to read line");

        let choice = cell.trim();

        let choice: Vec<&str> = choice.split("").filter(|x| !x.is_empty()).collect();

        let row =  match choice[0].to_lowercase().as_ref() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            _ => 0
        };

        let column = choice[1].parse::<usize>().unwrap_or(0);

        grid[row][column - 1] = player;

        player = match player {
            Value::Cross => Value::Naught,
            Value::Naught => Value::Cross,
            Value::None => Value::None,
        };

        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");

        print_grid(&mut grid);

    }
}
