use std::{
	fs::File,
	io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	let mut res: Vec<char> = Vec::new();
	let mut sum = 0;

	for line in reader.lines() {
		let s = String::from_utf8(line?.into_bytes()).unwrap();
		let (f, s) = s.split_at(s.len() / 2);

		for c in f.chars() {
			if s.contains(c) {
				res.push(c);
				break;
			}
		}
	}

    println!("{}", get_weight(res));

    let file = File::open("input_2.txt")?;
	let reader = BufReader::new(file);
	let mut res: Vec<char> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();
	let mut count = 0;

	for line in reader.lines() {
		let s = String::from_utf8(line?.into_bytes()).unwrap();
        count += 1;

        if count <= 3 {
            tmp.push(s);
        } 

        if count == 3 {
            for c in tmp[0].chars().into_iter() {
                if tmp[1].contains(c) && tmp[2].contains(c) {
                    res.push(c);
                    break;
                }
            }

            tmp = Vec::new();
            count = 0;
        }
	}

    println!("{}", get_weight(res));

	Ok(())
}

fn get_weight(res: Vec<char>) -> i32 {
    let mut sum = 0;

    let weight = vec![
		"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
		"s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
		"K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
	];

	for char in res.into_iter() {
		let index = weight
			.iter()
			.position(|&r| r == char.to_string().as_str())
			.unwrap();
		sum += index as i32 + 1;
	}

	sum
}