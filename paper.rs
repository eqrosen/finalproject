use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Paper {
    pub id: u32,
}

pub fn read_papers(filename: &str) -> HashMap<u32, Paper> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut papers = HashMap::new();
    for line in reader.lines().skip(4) {
        let line = line.unwrap();
        let mut values = line.split("\t");
        let id = values.next().unwrap().parse::<u32>().unwrap();
        let paper = Paper {id };
        papers.insert(id, paper);
    }
    papers
}

