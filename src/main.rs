#![feature(test)]

extern crate test;

use std::fs;
use std::path::Path;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::time::Instant;

#[cfg(test)] mod tests;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
	first_name: String,
	last_name: String,
	age: u8,
	phones: Vec<String>,
}

impl Person {
	fn new(first_name: &str, last_name: &str, age: u8, phones: &Vec<String>) -> Person {
		Person { 
			first_name: first_name.to_string(), 
			last_name: last_name.to_string(), 
			age: age, 
			phones: phones.to_vec(), 
		}
	}
}

fn create_data(sz: usize) -> Vec<Person> {
	let mut rng = rand::thread_rng();
	let mut data = Vec::<Person>::new();
	for i in 0..sz {
		let mut phones = Vec::<String>::new();
		for j in 0..i % 3 {
			phones.push(format!("+7 (111) 123-45-6{}", j.to_string()));
		}
		let first_name = "John".to_string();
		let last_name = format!("Dohn #{}", i.to_string());
		let character = Person::new(&first_name, &last_name, 20 + rng.gen_range(0, 40), &phones);
		data.push(character);
	}
	data
}

fn write_to_file<P: AsRef<Path>>(file_path: P, data: &str) {
	fs::write(file_path, data).expect("Error writing to file");
}

fn read_from_file<P: AsRef<Path>>(file_path: P) -> Result<String, String> {
	let contents = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
	Ok(contents)
}

fn create_data_and_write_to_file() {
	//100 data
    let data = create_data(100);
	let string_data = serde_json::to_string(&data).expect("Error convering to string");
	write_to_file("data100.txt", &string_data);
	
	//1K data
    let data = create_data(1000);
	let string_data = serde_json::to_string(&data).expect("Error convering to string");
	write_to_file("data1K.txt", &string_data);
	
	//10K data
    let data = create_data(10000);
	let string_data = serde_json::to_string(&data).expect("Error convering to string");
	write_to_file("data10K.txt", &string_data);	
	
	//1M data
    let data = create_data(1000000);
	let string_data = serde_json::to_string(&data).expect("Error convering to string");
	write_to_file("data1M.txt", &string_data);
}

pub fn x(data: &str, unTyped: bool) -> u128 {
	let now = Instant::now();
	if unTyped {
		let obj: Value = serde_json::from_str(data).expect("error deserializing");
	}
	else {
		let obj: Vec::<Person> = serde_json::from_str(data).expect("error deserializing");
	}
	now.elapsed().as_micros()
}

fn main() {
	//create_data_and_write_to_file();
	//Parsing JSON as strongly typed data structures
	println!("Parsing JSON as serde_json::Value type");
	//Deserializing vec of 100 elements
	let data = read_from_file("data/data100.txt").expect("error reading file");
	println!("Done vec100. Time taken {} microsecs", x(&data, true));
	
	//Deserializing vec of 1000 elements
	let data = read_from_file("data/data1K.txt").expect("error reading file");
	println!("Done vec1000. Time taken {} microsecs", x(&data, true));	
	
	//Deserializing vec of 10000 elements
	let data = read_from_file("data/data10K.txt").expect("error reading file");
	println!("Done vec10000. Time taken {} microsecs", x(&data, true));	

	//Deserializing vec of 1000000 elements
	let data = read_from_file("data/data1M.txt").expect("error reading file");
	println!("Done vec1000000. Time taken {} microsecs", x(&data, true));	
	
	//Parsing JSON as strongly typed data structures
	println!("Parsing JSON as strongly typed data structures");
	//Deserializing vec of 100 elements
	let data = read_from_file("data/data100.txt").expect("error reading file");
	println!("Done vec100. Time taken {} microsecs", x(&data, false));
	
	//Deserializing vec of 1000 elements
	let data = read_from_file("data/data1K.txt").expect("error reading file");
	println!("Done vec1000. Time taken {} microsecs", x(&data, false));	
	
	//Deserializing vec of 10000 elements
	let data = read_from_file("data/data10K.txt").expect("error reading file");
	println!("Done vec10000. Time taken {} microsecs", x(&data, false));	

	//Deserializing vec of 1000000 elements
	let data = read_from_file("data/data1M.txt").expect("error reading file");
	println!("Done vec1000000. Time taken {} microsecs", x(&data, false));		
}
