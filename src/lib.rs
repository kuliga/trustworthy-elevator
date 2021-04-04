use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Index;

struct Load {
        min: u16,
	max: u16,
	cur: u16
}

struct Floor {
	min: i16,
	max: i16,
	cur: i16
}

enum Direction {
        UP,
        DOWN,
        NONE
}

pub struct Elevator {
        load: Load,
        floor: Floor,
        dir: Direction,
        reqs: HashMap<i16, u8>
}

//associated functions
impl Elevator {
	pub fn new() -> Elevator {
		Elevator {
			load: Load {min: 0, max: 0, cur: 0},
			floor: Floor {min: 0, max: 0, cur: 0},
			dir: Direction::NONE,
			reqs: HashMap::new()
		}
	}
}

//methods
impl Elevator {
pub fn panel(&mut self) {
	loop {
		println!("Enter floor number: ");
		let mut get_num = String::new();
		io::stdin()
			.read_line(&mut get_num)
			.expect("Failed to enter floor number!");
		
		let get_num: i16 = match get_num.trim().parse() {
			Ok(num) => num,
			Err(_) => break
		};
		
		if self.reqs.contains_key(&get_num) {
			let val = *self.reqs.index(&get_num);
			self.reqs.insert(get_num, val + 1);
		} else {
			self.reqs.insert(get_num, 1);
		}
	}
}
}