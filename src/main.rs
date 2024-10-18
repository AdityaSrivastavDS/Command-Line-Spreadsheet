mod spreadsheet;

use crate::spreadsheet::Spreadsheet;
use std::io::{self, Write};

fn main() {
    let mut sheet = Spreadsheet::new(5, 5); // 5x5 grid

    loop {
        println!("Choose an option:");
        println!("1. Set a cell value");
        println!("2. View spreadsheet");
        println!("3. Evaluate a cell");
        println!("4. Save spreadsheet");
        println!("5. Load spreadsheet");
        println!("6. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                let (row, col) = get_cell_position();
                let mut value = String::new();
                print!("Enter value: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut value).unwrap();
                sheet.set_value(row, col, value.trim().to_string());
            }
            "2" => {
                sheet.display();
            }
            "3" => {
                let (row, col) = get_cell_position();
                let result = sheet.evaluate_cell(row, col);
                println!("Evaluated result: {}", result);
            }
            "4" => {
                let mut file_name = String::new();
                print!("Enter file name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut file_name).unwrap();
                sheet.save_to_file(file_name.trim()).unwrap();
                println!("Spreadsheet saved.");
            }
            "5" => {
                let mut file_name = String::new();
                print!("Enter file name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut file_name).unwrap();
                sheet.load_from_file(file_name.trim()).unwrap();
                println!("Spreadsheet loaded.");
            }
            "6" => {
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

fn get_cell_position() -> (usize, usize) {
    let mut row = String::new();
    let mut col = String::new();

    print!("Enter row: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut row).unwrap();

    print!("Enter column: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut col).unwrap();

    let row = row.trim().parse::<usize>().unwrap_or(0);
    let col = col.trim().parse::<usize>().unwrap_or(0);

    (row, col)
}
