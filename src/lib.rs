use std::fmt;
use std::io;
use std::ops::Index;

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
        floor: Floor,
        dir: Direction,
        reqs: Vec<i16>
}

//associated functions
impl Elevator {
	pub fn new() -> Elevator {
		Elevator {
			floor: Floor {min: 0, max: 0, cur: 0},
			dir: Direction::NONE,
			reqs: Vec::<i16>::new()
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
			self.reqs.push(get_num);
		}
	}

	pub fn go(&mut self) {
		
	}
}