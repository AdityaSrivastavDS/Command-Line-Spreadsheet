use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;

pub struct Spreadsheet {
    data: Vec<Vec<String>>, // Store cell data as strings (can be numbers or formulas)
    rows: usize,
    cols: usize,
}

impl Spreadsheet {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec!["".to_string(); cols]; rows],
            rows,
            cols,
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: String) {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
        }
    }

    pub fn get_value(&self, row: usize, col: usize) -> Option<&String> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    pub fn display(&self) {
        println!("Spreadsheet:");
        for row in &self.data {
            for cell in row {
                print!("{}\t", cell);
            }
            println!();
        }
    }

    pub fn evaluate_cell(&self, row: usize, col: usize) -> f64 {
        if let Some(cell) = self.get_value(row, col) {
            // If it's a number, return it directly
            if let Ok(num) = f64::from_str(cell) {
                return num;
            }
            // If it's a formula (e.g., "=2+3"), evaluate it
            if cell.starts_with('=') {
                let formula = &cell[1..];
                return self.evaluate_formula(formula);
            }
        }
        0.0
    }

    fn evaluate_formula(&self, formula: &str) -> f64 {
        // For simplicity, this will only handle addition and subtraction
        let mut result = 0.0;
        let mut current_number = String::new();
        let mut last_operator = '+';

        for ch in formula.chars() {
            if ch.is_digit(10) || ch == '.' {
                current_number.push(ch);
            } else if ch == '+' || ch == '-' {
                if let Ok(num) = current_number.parse::<f64>() {
                    result = match last_operator {
                        '+' => result + num,
                        '-' => result - num,
                        _ => result,
                    };
                }
                current_number.clear();
                last_operator = ch;
            }
        }

        if let Ok(num) = current_number.parse::<f64>() {
            result = match last_operator {
                '+' => result + num,
                '-' => result - num,
                _ => result,
            };
        }

        result
    }

    pub fn save_to_file(&self, file_name: &str) -> io::Result<()> {
        let mut file = File::create(file_name)?;
        for row in &self.data {
            let line = row.join(",");
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, file_name: &str) -> io::Result<()> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let cells: Vec<String> = line.split(',').map(String::from).collect();
            if i < self.rows {
                for (j, cell) in cells.into_iter().enumerate() {
                    if j < self.cols {
                        self.set_value(i, j, cell);
                    }
                }
            }
        }
        Ok(())
    }
}
