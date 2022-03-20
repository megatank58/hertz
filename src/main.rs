use std::env;
use std::fs;

pub mod lib;
use lib::{Tokens, Variable};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut data_store: Vec<Tokens> = vec![];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents
        .lines()
        .map(|string| string.parse().unwrap())
        .collect();
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let keyword = tokens[0];

        match keyword {
            "let" => data_store = parse_variable(tokens, &data_store),
            _ => println!("None"),
        }
    }

    println!("{:?}", data_store)
}

fn parse_variable(tokens: Vec<&str>, data_store: &Vec<Tokens>) -> Vec<Tokens> {
    let key = tokens[1];
    let value = tokens[3];

    let mut data_storage = data_store.to_vec();

    data_storage.push(Tokens::Var(Variable::new(
        key.to_string(),
        value.to_string(),
    )));

    data_storage
}
