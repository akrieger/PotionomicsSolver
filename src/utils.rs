use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_export]
macro_rules! regex {
    ($regex:literal) => {
        regex::Regex::new($regex).unwrap()
    };
}

pub fn get_input(filename: &str) -> Box<dyn Iterator<Item = String>> {
    Box::new(
        BufReader::new(File::open("input/".to_owned() + filename).unwrap())
            .lines()
            .map(|l| l.unwrap()),
    )
}
