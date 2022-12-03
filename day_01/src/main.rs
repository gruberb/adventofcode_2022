use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut res = HashMap::new();
    let mut elfe = 1;
    let mut n = 0;


    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            res.insert(elfe, n);
            elfe = elfe + 1;
            n = 0;
        } else {
            n = n + line.unwrap().parse::<i32>().unwrap();
        }
    }

    let mut current = 0;
    let mut elfe = 0;

    for (key,value) in res.iter() {

        if value > &current {
            current = *value;
            elfe = *key
        }
    }

    println!("Elfe: {}, calories: {}", elfe, current);

    let mut v: Vec<i32> = res.into_values().collect();
    v.sort();
    println!("Sum top 3: {:?}", v.iter().rev().take(3).sum::<i32>());


    Ok(())
}
