use std::{
	cell::RefCell,
	rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct File {
	size: u32,
}

#[derive(Debug, Clone)]
struct Directory {
	parent: Option<Weak<RefCell<Directory>>>,
	name: String,
	files: Vec<File>,
	children: Vec<Rc<RefCell<Directory>>>,
}

#[derive(Debug, Clone)]
struct Hierachy {
	head: Rc<RefCell<Directory>>,
}

impl Hierachy {
	fn find_sizes(&self, directory: Option<Rc<RefCell<Directory>>>, result: &mut Vec<u32>) -> u32 {
		let directory = directory.unwrap_or_else(|| self.head.clone());
		let mut size = 0;
		for d in directory.borrow().children.iter() {
			size += self.find_sizes(Some(d.clone()), result);
		}
		let file_sum = directory.borrow().files.iter().map(|f| f.size).sum::<u32>();
		result.push(file_sum + size);
		file_sum + size
	}
}

fn main() {
	let contents =
		std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

	let root = Hierachy {
		head: Rc::new(RefCell::new(Directory {
			parent: None,
			name: "/".to_string(),
			files: Vec::new(),
			children: Vec::new(),
		})),
	};

	let mut current_directory = root.head.clone();

	for line in contents.lines().skip(2) {
		if line.starts_with("$ cd") {
			if (&line[5..]) != ".." {
				let new_node = current_directory
					.borrow()
					.children
					.iter()
					.find(|d| d.borrow().name == line[5..].to_string())
					.unwrap()
					.clone();
				current_directory = new_node;
			} else if (&line[5..]) == ".." {
				let new_node = current_directory.borrow().parent.as_ref().unwrap().clone();
				current_directory = new_node.upgrade().clone().unwrap();
			}
		} else if line.starts_with("$ ls") {
			// do nothing
		} else if line.starts_with("dir") {
			current_directory
				.borrow_mut()
				.children
				.push(Rc::new(RefCell::new(Directory {
					parent: Some(Rc::downgrade(&current_directory)),
					name: line[4..].to_string(),

					files: Vec::new(),
					children: Vec::new(),
				})));
		} else {
			let (size, _) = line.split_once(' ').unwrap();
			current_directory.borrow_mut().files.push(File {
				size: size.parse::<u32>().unwrap(),
			})
		}
	}

	let mut sizes = Vec::new();
	let _total_size = root.find_sizes(None, &mut sizes);
	let part_01 = sizes
		.into_iter()
		.filter(|&size| size <= 100000)
		.sum::<u32>();

	println!("Part 1: {:#?}", part_01);

    let mut sizes = Vec::new();
	let total_size = root.find_sizes(None, &mut sizes);
    let free_space = 70000000 as u32 - total_size;
	let part_02 = sizes
		.into_iter()
		.filter(|&size| free_space + size >= 30000000)
        .min()
		.unwrap();

	println!("Part 2: {}", part_02);
}
