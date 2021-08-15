use std::fmt;
// use std::io;
// use rand::Rng;

struct Grid (Row, Row, Row);

struct Row {
    position: RowPosition,
    values: [Option<Value>; 3],
}

enum Value {
    Naught,
    Cross,
    None
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Naught => write!(f, "O"),
            Value::Cross => write!(f, "X"),
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

fn print_row_middle(values: &[Option<Value>; 3]) {
    println!("|          |          |          |");
    println!("|     {}    |     {}    |     {}    |", values[0].as_ref().unwrap(), values[1].as_ref().unwrap(), values[2].as_ref().unwrap());
    println!("|          |          |          |");
}

fn print_row(row: Row) {
    match row.position {
        RowPosition::Top => { 
            print_row_border();
            print_row_middle(&row.values);
        },
        RowPosition::Middle => {
            print_row_border();
            print_row_middle(&row.values);
            print_row_border();
        },
        RowPosition::Bottom => {
            print_row_middle(&row.values);
            print_row_border();
        }
    }
}

fn print_grid(grid: &[[Option<Value>; 3]; 3]) {
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
    let grid: [[Option<Value>; 3]; 3] = [
        [None, None, None],
        [None, None, None],
        [None, None, None],
    ];
    print_grid(&grid);
}
