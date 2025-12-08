use std::env::args;
use std::error;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

fn count_laser_division(
    input: &Vec<String>,
    row: usize,
    size_columns: usize,
    results: &mut Vec<u64>,
) {
    for column in 0..size_columns {
        if results[column] == 0 {
            continue;
        } else if (column > 0 && &input[row][(column - 1)..column] == "^")
            && (column < (size_columns - 1) && &input[row][(column + 1)..(column + 2)] == "^")
            && &input[row - 1][(column - 1)..column] == "|"
            && &input[row - 1][(column + 1)..(column + 2)] == "|"
        {
            results[column - 1] += results[column];
            results[column + 1] += results[column];
            if &input[row - 1][column..(column + 1)] != "|" {
                results[column] = 0;
            }
        } else if column > 0
            && &input[row][(column - 1)..column] == "^"
            && &input[row - 1][(column - 1)..column] == "|"
        {
            results[column - 1] += results[column];
            if &input[row - 1][column..(column + 1)] != "|" {
                results[column] = 0;
            }
        } else if column < (size_columns - 1)
            && &input[row][(column + 1)..(column + 2)] == "^"
            && &input[row - 1][(column + 1)..(column + 2)] == "|"
        {
            results[column + 1] += results[column];
            if &input[row - 1][column..(column + 1)] != "|" {
                results[column] = 0;
            }
        }
    }
}

fn get_modified_vector(
    input: &Vec<String>,
    size_rows: usize,
    size_columns: usize,
    results: &mut Vec<u64>,
) {
    for row in (1..size_rows - 1).rev() {
        count_laser_division(input, row, size_columns, results);
    }
}

fn get_result_part_two(input: &Vec<String>, size_rows: usize, size_columns: usize) -> u64 {
    let mut results: Vec<u64> = Vec::new();
    let mut result_part_two: u64 = 0;
    let last_line = match input.last() {
        Some(line) => line.to_string(),
        None => {
            return 0;
        }
    };
    for character in last_line.chars() {
        if character == '|' {
            results.push(1);
        } else {
            results.push(0);
        }
    }
    get_modified_vector(input, size_rows, size_columns, &mut results);
    for number in &results {
        result_part_two += *number;
    }
    return result_part_two;
}

fn fill_map_with_beams(
    input: &mut Vec<String>,
    pos_row: usize,
    pos_column: usize,
    size_rows: usize,
    size_columns: usize,
) -> u64 {
    let mut result_one: u64 = 0;
    if pos_row >= size_rows || pos_column >= size_columns {
        return result_one;
    }
    let character: &str = &input[pos_row][pos_column..pos_column + 1];
    if character == "." {
        input[pos_row].replace_range(pos_column..pos_column + 1, "|");
        return fill_map_with_beams(input, pos_row + 1, pos_column, size_rows, size_columns);
    } else if character == "^" {
        if pos_column != 0 {
            result_one +=
                fill_map_with_beams(input, pos_row, pos_column - 1, size_rows, size_columns);
        }
        result_one += fill_map_with_beams(input, pos_row, pos_column + 1, size_rows, size_columns);
        result_one += 1;
    }
    return result_one;
}

fn get_position_source(input: &Vec<String>, amount_rows: usize) -> (usize, usize) {
    let mut row: usize = 0;
    let mut column: usize = 0;
    while row < amount_rows {
        match input[row].find('S') {
            Some(index) => {
                column = index;
                break;
            }
            None => row += 1,
        };
    }
    return (row, column);
}

fn get_results(input: &mut Vec<String>) -> (u64, u64) {
    let result_part_one: u64;
    let result_part_two: u64;
    let size_rows: usize = input.len();
    let size_columns: usize = input[0].len();
    let (pos_row, pos_column) = get_position_source(input, size_rows);
    result_part_one = fill_map_with_beams(input, pos_row + 1, pos_column, size_rows, size_columns);
    result_part_two = get_result_part_two(input, size_rows, size_columns);
    return (result_part_one, result_part_two);
}

fn get_map_as_owned(original_map: Vec<&str>) -> Vec<String> {
    let mut result_vector: Vec<String> = Vec::new();
    for line in original_map {
        if line.len() != 0 && line.chars().filter(|s| s.is_whitespace()).count() == 0 {
            result_vector.push(line.to_string());
        }
    }
    return result_vector;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let argv: Vec<String> = args().collect();
    let mut map_grid: Vec<String>;
    if argv.len() == 1 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Not enough parameters passsed",
        )));
    }
    if argv.len() != 2 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Too many parameters",
        )));
    }
    let file_string: String = read_to_string(&argv[1])?;
    let input_as_references: Vec<&str> = file_string.split('\n').collect();
    if input_as_references.len() == 0 {
        return Err(Box::new(Error::new(ErrorKind::Other, "The file is empty")));
    }
    map_grid = get_map_as_owned(input_as_references);
    let (result_part_one, result_part_two): (u64, u64) = get_results(&mut map_grid);
    println!("Result of part one: {result_part_one}");
    println!("Result of part two: {result_part_two}");
    Ok(())
}
