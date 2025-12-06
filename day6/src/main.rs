use std::env::args;
use std::error;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

fn get_result_part_two(vector_with_number: &Vec<Vec<u64>>, operations_vector: &Vec<&str>) -> u64 {
    let mut result: u64 = 0;
    let mut result_mul: u64;
    let mut result_sum: u64;
    let size_operations: usize = operations_vector.len();
    let mut number_operation: usize = 0;
    let mut length_vector_number: usize;
    let mut index_vector_numbers: usize;
    while number_operation < size_operations {
        result_mul = 1;
        result_sum = 0;
        let operation_string: String = operations_vector[number_operation].to_string();
        length_vector_number = vector_with_number[number_operation].len();
        index_vector_numbers = 0;
        while index_vector_numbers < length_vector_number - 1 {
            if index_vector_numbers == 0 && operation_string == "+" {
                result_sum += vector_with_number[number_operation][index_vector_numbers]
                    + vector_with_number[number_operation][index_vector_numbers + 1];
            } else if operation_string == "+" {
                result_sum += vector_with_number[number_operation][index_vector_numbers + 1];
            } else if index_vector_numbers == 0 && operation_string == "*" {
                result_mul *= vector_with_number[number_operation][index_vector_numbers]
                    * vector_with_number[number_operation][index_vector_numbers + 1];
            } else {
                result_mul *= vector_with_number[number_operation][index_vector_numbers + 1];
            }
            index_vector_numbers += 1;
        }
        if operation_string == "+" {
            result += result_sum;
        } else {
            result += result_mul;
        }
        number_operation += 1;
    }
    return result;
}

fn get_biggest_length_column(original_numbers: &Vec<Vec<String>>, column: usize) -> usize {
    let mut biggest_length: usize = 0;
    for line in original_numbers {
        if biggest_length < line[column].len() {
            biggest_length = line[column].len();
        }
    }
    return biggest_length;
}

fn get_biggest_length_row(input: &Vec<String>) -> usize {
    let mut length: usize = 0;
    let mut length_from_string: usize;
    for line in input {
        length_from_string = line.len();
        if length < length_from_string {
            length = line.len();
        }
    }
    return length;
}

fn get_numbers_vectors_p2(
    original_numbers: &Vec<Vec<String>>,
    columns: usize,
    amount_rows: usize,
) -> Result<Vec<u64>, Box<dyn error::Error>> {
    let mut result_vector: Vec<u64> = Vec::new();
    let biggest_length: usize = get_biggest_length_column(original_numbers, columns);
    let mut rows: usize;
    let mut index: usize = 0;
    let mut temp_result: u64;
    while index < biggest_length {
        rows = 0;
        temp_result = 0;
        while rows < amount_rows {
            if original_numbers[rows][columns].len() <= index {
                rows += 1;
                continue;
            }
            let temp_number_slice: u64 =
                original_numbers[rows][columns][index..index + 1].parse()?;
            if temp_number_slice == 0 {
                rows += 1;
                continue;
            }
            temp_result = temp_result * 10 + temp_number_slice;
            rows += 1;
        }
        result_vector.push(temp_result);
        index += 1;
    }
    return Ok(result_vector);
}

fn get_vector_for_part_2(
    original_numbers: &Vec<Vec<String>>,
) -> Result<Vec<Vec<u64>>, Box<dyn error::Error>> {
    let amount_columns: usize = original_numbers[0].len();
    let amount_rows: usize = original_numbers.len();
    let mut columns: usize = 0;
    let mut result_vector: Vec<Vec<u64>> = Vec::new();
    while columns < amount_columns {
        result_vector.push(get_numbers_vectors_p2(
            original_numbers,
            columns,
            amount_rows,
        )?);
        columns += 1;
    }
    return Ok(result_vector);
}

fn prepare_vector_for_part_two(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut temp_vector_lines: Vec<String> = Vec::new();
    let mut result_vector: Vec<Vec<String>> = Vec::new();
    let mut vector_for_slice: Vec<usize> = Vec::new();
    for line in input {
        temp_vector_lines.push(line.replace(" ", "0"));
    }
    let max_index: usize = get_biggest_length_row(input);
    let mut index: usize = 0;
    let mut row: usize;
    let max_rows: usize = input.len();
    let mut temp: usize;
    while index < max_index {
        row = 0;
        temp = 0;
        while row < max_rows {
            if index >= temp_vector_lines[row].len() {
                row += 1;
                continue;
            }
            if &temp_vector_lines[row][index..index + 1] == "0" {
                temp += 1;
            }
            row += 1;
        }
        if temp == max_rows {
            vector_for_slice.push(index);
        }
        index += 1;
    }
    vector_for_slice.push(get_biggest_length_row(&temp_vector_lines));
    for line in &temp_vector_lines {
        let mut temp_vector: Vec<String> = Vec::new();
        let mut start_slice: usize = 0;
        for index_slice in &vector_for_slice {
            if line.len() < *index_slice {
                temp_vector.push(line[start_slice..line.len()].to_string());
            } else {
                temp_vector.push(line[start_slice..*index_slice].to_string());
            }
            start_slice = *index_slice + 1;
        }
        result_vector.push(temp_vector);
    }
    return result_vector;
}

fn get_result_part_one(
    operations: &Vec<&str>,
    numbers: &Vec<Vec<String>>,
) -> Result<u64, Box<dyn error::Error>> {
    let mut result: u64 = 0;
    let amount_columns: usize = operations.len();
    let amount_rows: usize = numbers.len();
    let mut column: usize = 0;
    let mut row: usize;
    let mut result_mul: u64;
    let mut result_plus: u64;
    while column < amount_columns {
        row = 0;
        result_mul = 1;
        result_plus = 0;
        let operation: String = operations[column].to_owned();
        while row < amount_rows - 1 {
            let number1: u64 = numbers[row][column].parse()?;
            let number2: u64 = numbers[row + 1][column].parse()?;
            if operation == "+" && row == 0 {
                result_plus += number1 + number2;
            } else if operation == "+" {
                result_plus += number2;
            } else if operation == "*" && row == 0 {
                result_mul *= number1 * number2;
            } else {
                result_mul *= number2;
            }
            row += 1;
        }
        if operation == "+" {
            result += result_plus;
        } else {
            result += result_mul;
        }
        column += 1;
    }
    Ok(result)
}

fn validate_length_op_and_numbers(
    operations: &Vec<&str>,
    numbers: &Vec<Vec<String>>,
) -> Result<(), Box<dyn error::Error>> {
    let length_operation: usize = operations.len();
    let mut length_numbers: usize = numbers[0].len();
    for line in numbers {
        if length_numbers != line.len() {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Different amount of numbers between lines",
            )));
        }
        length_numbers = line.len();
    }
    if length_numbers != length_operation {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Different amount of numbers and operations",
        )));
    }
    Ok(())
}

fn get_numbers_per_line(line_to_parse: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let temp_vector: Vec<&str> = line_to_parse.split_whitespace().collect();
    for number in temp_vector {
        result.push(number.to_string());
    }
    return result;
}

fn validate_operations(operations_vectors: &Vec<&str>) -> Result<(), Box<dyn error::Error>> {
    for symbol in operations_vectors {
        if symbol.to_owned() != "+" && symbol.to_owned() != "*" {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Invalid mathemathical operation",
            )));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let argv: Vec<String> = args().collect();
    let mut vector_numbers_lines: Vec<String> = Vec::new();
    let mut operations_string: String = "".to_owned();
    let mut vector_numbers_2d: Vec<Vec<String>> = Vec::new();
    let result_part_one: u64;
    let result_part_two: u64;
    let operations_vector: Vec<&str>;
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
    let mut index: usize = 0;
    let length: usize = input_as_references.len();
    if length == 0 {
        return Err(Box::new(Error::new(ErrorKind::Other, "The file is empty")));
    }
    while index < length {
        if index == length - 1 {
            operations_string = input_as_references[index].to_string();
        } else {
            vector_numbers_lines.push(input_as_references[index].to_string());
        }
        index += 1;
    }
    operations_vector = operations_string.split_whitespace().collect();
    validate_operations(&operations_vector)?;
    for line_operations in &vector_numbers_lines {
        vector_numbers_2d.push(get_numbers_per_line(line_operations));
    }
    validate_length_op_and_numbers(&operations_vector, &vector_numbers_2d)?;
    result_part_one = get_result_part_one(&operations_vector, &vector_numbers_2d)?;
    println!("The result of part one is {result_part_one}");
    let string_vector_part_two: Vec<Vec<String>> =
        prepare_vector_for_part_two(&vector_numbers_lines);
    let vector_part_2: Vec<Vec<u64>> = get_vector_for_part_2(&string_vector_part_two)?;
    result_part_two = get_result_part_two(&vector_part_2, &operations_vector);
    println!("The result of part one is {result_part_two}");
    Ok(())
}
