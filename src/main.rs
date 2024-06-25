use std::io;

fn main() {
    println!("Ceaser Cypher");

    println!("How many number should shift:");
    let shifting_number = 5;

    let mut u_input = String::new();

    println!("Please enter a text that need to encrypted:");
    io::stdin()
               .read_line(&mut u_input)
               .expect("Failed to read line");
    let upper_case_letters = ['A', 'B', 'C', 'D', 'E', 
                                          'F', 'G', 'H', 'I', 'J', 
                                          'K', 'L', 'M', 'N', 'O', 
                                          'P', 'Q', 'R', 'S', 'T', 
                                          'U', 'V', 'W', 'X', 'Y', 'Z'];
    let lower_case_letters = ['a', 'b', 'c', 'd', 'e', 
                                          'f', 'g', 'h', 'i', 'j', 
                                          'k', 'l', 'm', 'n', 'o', 
                                          'p', 'q', 'r', 's', 't', 
                                          'u', 'v', 'w', 'x', 'y', 'z'];

    
    let char_vec: Vec<char> = u_input.chars().collect();
    let mut char_array: Vec<char> = Vec::new();
    for c in char_vec{
        char_array.push(c);
    }

    for char in char_array {
        if let Some(index) = lower_case_letters.iter().position(|&char2| char2 == char) {
            println!("Character '{}' found at index {} in the second array", char, index);
            let shifted = index + shifting_number;
        }
    }
    println!("{}", upper_case_letters[5]);
    println!("{}", lower_case_letters[5]);
    println!("addition of 2 and 3 is {}", shifting_number);
}


