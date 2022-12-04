use std::{
	fs::File,
	io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
	let reader = BufReader::new(file);

	let mut part_1 = 0;
    let mut part_2 = 0;

	for line in reader.lines() {
		let s = String::from_utf8(line?.into_bytes()).unwrap();
        let ranges: Vec<&str> = s.split(",").collect();

        let first_range: Vec<&str> = ranges[0].split("-").collect();
        let second_range: Vec<&str> = ranges[1].split("-").collect();

        let fd = first_range[0].parse::<i32>().unwrap();
        let fu = first_range[1].parse::<i32>().unwrap();

        let sd = second_range[0].parse::<i32>().unwrap();
        let su = second_range[1].parse::<i32>().unwrap();

        if fd <= sd && fu >= su || sd <= fd && su >= fu {
            part_1 += 1;
        }

        let fr: Vec<i32> = (fd..=fu).collect();
        let sr: Vec<i32> = (sd..=su).collect();

        for i in fr.iter() {
            println!("Check number {} in {:?}", i, sr);
            if sr.contains(&i) {
                println!("Number {:?} is in {:?}", i, sr);
                part_2 += 1;
                break;
            }
        }
	}

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

	Ok(())
}
