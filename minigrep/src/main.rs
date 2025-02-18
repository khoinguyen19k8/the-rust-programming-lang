use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("File content:\n{contents}")
}

fn parse_config(args: &Vec<String>) -> (&str, &str) {

}

