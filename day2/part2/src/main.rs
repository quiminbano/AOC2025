use std::error;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::str::Split;

fn get_number_length(mut number: u64) -> u64 {
    if number <= 9 {
        return 1;
    }
    let mut length: u64 = 0;
    while number > 0 {
        number = number.div_euclid(10);
        length += 1;
    }
    length
}

fn check_other_invalid_cases(number: u64, length_number: u64) -> u64 {
    //Find a better way to do this, to optimize this function performance
    let number_as_string: String = number.to_string();
    let half_length: u64 = length_number.div_euclid(2);
    let mut slice_index: usize = 1;
    while slice_index <= half_length as usize {
        let slice_string = &number_as_string[0..slice_index];
        if number_as_string.trim_start_matches(slice_string).len() == 0 {
            return number;
        }
        slice_index += 1;
    }
    return 0;
}

fn is_invalid_id(number: u64, length_number: u64) -> bool {
    let number_to_divide: u64;
    let first_halve: u64;
    let second_halve: u64;

    number_to_divide = 10u64.pow(length_number.div_euclid(2) as u32);
    first_halve = number.div_euclid(number_to_divide);
    second_halve = number.rem_euclid(number_to_divide);
    if first_halve == second_halve {
        return true;
    }
    return false;
}

fn get_sum_invalid_ids(range_split_vector: &Vec<&str>) -> Result<u64, Box<dyn error::Error>> {
    let mut result_sum_invalid_ids: u64 = 0;
    let first_range_number: u64 = range_split_vector[0].parse()?;
    let last_range_number: u64 = range_split_vector[1].parse()?;

    for number in first_range_number..last_range_number + 1 {
        let length_number: u64 = get_number_length(number);
        if (length_number & 1) == 0 {
            let invalid_id: bool = is_invalid_id(number, length_number);
            if invalid_id {
                result_sum_invalid_ids += number;
            } else {
                result_sum_invalid_ids += check_other_invalid_cases(number, length_number);
            }
        } else {
            result_sum_invalid_ids += check_other_invalid_cases(number, length_number);
        }
    }
    Ok(result_sum_invalid_ids)
}

fn return_invalid_id_sum_per_range(range: &str) -> Result<u64, Box<dyn error::Error>> {
    let range_split: Split<'_, char> = range.split('-');
    let result_sum_invalid_id: u64;
    let mut range_split_vector: Vec<&str> = vec![];
    for splitted in range_split {
        range_split_vector.push(splitted);
    }
    if range_split_vector.len() != 2 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Range contains more than two numbers",
        )));
    }
    result_sum_invalid_id = get_sum_invalid_ids(&range_split_vector)?;
    Ok(result_sum_invalid_id)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file_content: String = read_to_string("input.txt")?;
    let id_collection: Split<'_, char> = file_content.split(',');
    let mut sum_invalid_ids: u64 = 0;
    for id_range in id_collection {
        sum_invalid_ids += return_invalid_id_sum_per_range(id_range)?;
    }
    println!("The sum of invalid ids is {sum_invalid_ids}");
    Ok(())
}
