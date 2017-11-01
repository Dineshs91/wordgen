mod structures;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::ascii::AsciiExt;

use structures::{Weight};

pub const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let sample_data = &read_data_from_file("resources/sample_text.txt");

    // Build next letter map before cleaning.
    let next_letter_map = build_next_letter_map(sample_data);

    let sample_data = &clean_data(sample_data);
    let alphabet_frequency_map = build_frequency_map(sample_data);
    println!("{:?}", next_letter_map);
    println!("The alphabets are {:?}", alphabet_frequency_map);
}

fn build_next_letter_map(sample_data: &str) -> HashMap<char, Vec<Weight>> {
    let mut next_alpha_map: HashMap<char, Vec<Weight>> = HashMap::new(); 
    let mut sample_data_iter = sample_data.chars().peekable();

    while let Some(mut alpha) = sample_data_iter.next() {
        let next_alpha = sample_data_iter.peek().unwrap_or(&' ');
        let mut next_alpha = next_alpha.clone();
        alpha = alpha.to_ascii_lowercase();
        next_alpha = next_alpha.to_ascii_lowercase();

        if alpha == ' ' || next_alpha == ' ' || !alpha.is_alphabetic() || !next_alpha.is_alphabetic() {
            continue;
        }

        let weight_vector: Vec<Weight> = match next_alpha_map.get(&alpha) {
            Some(vector) => vector.clone(),
            None => {
                vec![]
            }
        };

        let mut found = false;
        let mut new_vector: Vec<Weight> = vec![];
        for mut weighted_alpha in weight_vector.clone() {
            if weighted_alpha.character == next_alpha {
                weighted_alpha.weight += 1;
                found = true;
            } 
            new_vector.push(weighted_alpha);
        }

        if !found {
            let weight = Weight { character: next_alpha, weight: 1 };
            new_vector.push(weight);
        } 
        next_alpha_map.insert(alpha, new_vector);
    }

    next_alpha_map
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