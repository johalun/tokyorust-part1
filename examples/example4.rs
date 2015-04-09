#![feature(core)]
#![feature(collections)]
use std::iter::AdditiveIterator;

fn main() {

    let kanji_array : [String;3] = [
    		      String::from_str("Rust is fun."),
		      String::from_str("You can use High Order Functions."),
		      String::from_str("I want to learn more.")
		      ];
    let char_count = kanji_array.iter().map(|s : &String|s.len()).sum();
    println!("Character count: {}", char_count);
}