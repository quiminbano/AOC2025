use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut count_zeros: u32 = 0;
    let mut starting_position: i32 = 50;
    let file_path: &str = "input.txt";
    let file_content: fs::File = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open the file input.txt");
            return ExitCode::FAILURE;
        }
    };
    let file_by_line = BufReader::new(file_content);

    for line in file_by_line.lines() {
        let mut passed_by_zero: u32 = 0;
        let mut instruction_from_file: String = match line {
            Ok(instruction) => instruction,
            Err(_) => {
                eprintln!("Failed parsing the file");
                return ExitCode::FAILURE;
            }
        };
        let is_right: bool = instruction_from_file.starts_with('R');
        instruction_from_file.remove(0);
        let number_from_line: i32 = match instruction_from_file.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed converting the a line into a number");
                return ExitCode::FAILURE;
            }
        };
        if is_right {
            if starting_position + number_from_line >= 100 {
                passed_by_zero = (starting_position + number_from_line).div_euclid(100) as u32;
            }
            starting_position = (starting_position + number_from_line).rem_euclid(100);
        } else {
            passed_by_zero = number_from_line.div_euclid(100) as u32;
            if (starting_position - (number_from_line - passed_by_zero as i32 * 100) <= 0)
                && (starting_position != 0)
            {
                passed_by_zero += 1;
            }
            starting_position = (starting_position - number_from_line).rem_euclid(100);
        }
        count_zeros += passed_by_zero;
    }
    println!("The password is {count_zeros}");
    ExitCode::SUCCESS
}
