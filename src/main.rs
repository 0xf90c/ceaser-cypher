use std::collections::HashSet;
use std::io;

fn string_to_char_arr(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn add_to_char(c: char, num: i32) -> char {
    let new_code = c as u32 + num as u32;
    std::char::from_u32(new_code).unwrap_or(c)
}

fn subtract_to_char(c: char, num: i32) -> char {
    let new_code = c as u32 - num as u32;
    std::char::from_u32(new_code).unwrap_or(c)
}

fn main() {
    println!("Choose one option:");
    println!("1. Encrypt");
    println!("2. Decrypt");

    let mut options = String::new();
    io::stdin()
        .read_line(&mut options)
        .expect("Please enter a 1 or 2 option");

    println!("Enter shifting number:");

    let mut shifting_number = String::new();
    io::stdin()
        .read_line(&mut shifting_number)
        .expect("Please enter an integer");
    let shifting_number: i32 = shifting_number
        .trim()
        .parse()
        .expect("Please enter an integer");

    // array-1
    println!("Enter a plain text:");
    let mut plain_text = String::new();
    io::stdin()
        .read_line(&mut plain_text)
        .expect("Please enter a string");
    let input_arr: Vec<char> = string_to_char_arr(&*plain_text);

    // array-2
    let lower_case_letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let set1: HashSet<_> = input_arr.iter().cloned().collect();

    let indices: Vec<usize> = lower_case_letters
        .iter()
        .enumerate()
        .filter(|(_, &c)| set1.contains(&c))
        .map(|(i, _)| i)
        .collect();

    if options == "1" {
        let mut mod_arr1: Vec<char> = lower_case_letters.clone();
        for &ind in &indices {
            mod_arr1[ind] = add_to_char(lower_case_letters[ind], shifting_number);
        }
        let final_mod_string: String = mod_arr1.iter().collect();

        println!("encrypted string: {}", final_mod_string);
    } else if options == "2" {
        let mut mod_arr2: Vec<char> = lower_case_letters.clone();
        for &ind in &indices {
            mod_arr2[ind] = subtract_to_char(lower_case_letters[ind], shifting_number);
        }
        let final_mod_string: String = mod_arr2.iter().collect();

        println!("encrypted string: {}", final_mod_string);
    } else {
        println!("Invalid input");
    }
}
