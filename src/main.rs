use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let sample_data = &read_data_from_file("resources/sample_text.txt");
    //let sample_data = &sample_data;
    let sample_data = &clean_data(sample_data);
    let alphabet_frequency_map = build_frequency_map(sample_data);

    println!("The alphabets are {:?}", alphabet_frequency_map);
}

fn read_data_from_file(file_path: &str) -> String {
    let mut file_content = String::new();

    let mut f = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file {}, error: {}", file_path, e)
    };

    f.read_to_string(&mut file_content).unwrap();
    file_content
}

fn clean_data<'a>(sample_data: &'a str) -> String {
    // Clean the input data, removing whitespaces and symbols.
    let mut cleaned_data = String::new();

    for character in sample_data.chars() {
        if is_alphabet(character) {
            cleaned_data.push(character);
        }
    }

    cleaned_data
}

fn is_alphabet(character: char) -> bool {
    for alpha in ALPHABETS.chars() {
        if character == alpha {
            return true;
        }
    }

    false
}

fn build_frequency_map(sample_data: &str) -> HashMap<char, i32> {
    let mut alphabet_frequency_map: HashMap<char, i32> = HashMap::new();

    for alpha in sample_data.chars() {
        let count = match alphabet_frequency_map.get(&alpha) {
            Some(val) => val + 1,
            None => 1
        };

        alphabet_frequency_map.insert(alpha, count);
    }

    alphabet_frequency_map
}