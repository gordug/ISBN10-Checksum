extern crate regex;

use std::env;
use regex::Regex;

fn main() {
    let args_array: Vec<String> = env::args().collect();
    if args_array.len() as i8 > 1{
        // Run from cargo
        println!("{}", get_checksum(&args_array[1]))
    }
    else {
        // Run from binary
        println!("{}",get_checksum(&args_array[0]))
    }
}

///
///
/// # Arguments
///
/// * `isbn`: ISBN Number in 1-23-456789-x format with x as placeholder for checksum
///
/// returns: ISBN Checksum digit, -1 if invalid format
///
/// # Examples
///Valid Formats
/// ``` 8-17-525766-x -> 0
///
/// ``` 9-99-215810-x -> 7
/// Invalid Format
/// ``` 999215810x -> -1
fn get_checksum (isbn: &str) -> i32 {
    let isbn_format = Regex::new(r"[0-9]-[0-9]{2}-[0-9]{6}-x").unwrap();
    if !isbn_format.is_match(&isbn) {return -1}
    let digits = isbn.chars().filter_map(|c| c.to_digit(10));
    let mut checksum:i32 = 0;
    for (index, digit ) in digits.enumerate() {
        checksum += digit as i32 * (index as i32 + 1);
    }
    let result:i32 = checksum % 11;
    result
}

#[cfg(test)]
mod tests{
    use crate::get_checksum;

    #[test]
    fn valid_format_returns_0(){
        assert_eq!(0, get_checksum("8-17-525766-x"))
    }

    #[test]
    fn valid_format_returns_7(){
        assert_eq!(7, get_checksum("9-99-215810-x"))
    }

    #[test]
    fn invalid_format_returns_neg_1(){
        assert_eq!(-1, get_checksum("999215810x"))
    }
}
