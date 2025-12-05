use std::env;
use std::error;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

fn get_result_second_part(range_iterators: &Vec<(u64, u64)>) -> u64 {
    let mut result: u64 = 0;
    let mut cleaned_up_range: Vec<(u64, u64)> = vec![];
    let length: usize = range_iterators.len();
    let mut index: usize = 0;

    while index < length {
        if index == 0 {
            cleaned_up_range.push(range_iterators[index]);
            index += 1;
            continue;
        }
        let last: &mut (u64, u64) = cleaned_up_range.last_mut().unwrap();
        if range_iterators[index].0 > last.1 {
            cleaned_up_range.push(range_iterators[index]);
        } else if range_iterators[index].0 >= last.0 && range_iterators[index].1 <= last.1 {
            index += 1;
            continue;
        } else {
            last.1 = range_iterators[index].1;
        }
        index += 1;
    }
    for range in cleaned_up_range {
        result += (range.1 - range.0) + 1;
    }
    return result;
}

fn get_result_part1(
    range_iterators: &Vec<(u64, u64)>,
    ids: &str,
) -> Result<u64, Box<dyn error::Error>> {
    let mut result: u64 = 0;
    let split_ids = ids.split('\n');
    for id in split_ids {
        let id_numeric: u64 = id.parse()?;
        for tuple_ranges in range_iterators {
            if id_numeric >= tuple_ranges.0 && id_numeric <= tuple_ranges.1 {
                result += 1;
                break;
            }
        }
    }
    Ok(result)
}

fn get_range_iterators(ranges: &str) -> Result<Vec<(u64, u64)>, Box<dyn error::Error>> {
    let mut tuple_result: Vec<(u64, u64)> = vec![];
    let first_split = ranges.split('\n');
    for ranges in first_split {
        let second_split: Vec<&str> = ranges.split("-").collect();
        if second_split.len() != 2 {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Wrong format for range1",
            )));
        }
        let first_range: u64 = second_split[0].parse()?;
        let second_range: u64 = second_split[1].parse()?;
        if second_range < first_range {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Wrong format for range2",
            )));
        }
        tuple_result.push((first_range, second_range));
    }
    tuple_result.sort_by(|x, y| x.0.cmp(&y.0));
    return Ok(tuple_result);
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input: Vec<String> = env::args().collect();
    if input.len() == 1 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Not enough parameters passsed",
        )));
    }
    if input.len() != 2 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Too many parameters",
        )));
    }
    let file_content: String = read_to_string(input[1].as_str())?;
    let (ranges, ids) = match file_content.split_once("\n\n") {
        Some(tuple) => tuple,
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Invalid input in read file",
            )));
        }
    };
    let range_iterators = get_range_iterators(ranges)?;
    let result1: u64 = get_result_part1(&range_iterators, ids)?;
    println!("The result of part 1 is {result1}");
    let result2: u64 = get_result_second_part(&range_iterators);
    println!("The result of part 2 is {result2}");
    Ok(())
}
