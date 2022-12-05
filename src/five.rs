use crate::get_puzzle_input;

pub fn five() {
	let inp = get_puzzle_input("input/five_input.txt");
	let crates = inp[0].to_owned();
	let rows: [Vec<bool>; 9];

	let instruct = inp[1].to_owned();
	{
		let mut spaces_count = 0;
		for c in crates.chars() {
			if c == ' ' {
				spaces_count += 1;
			}
		}
	}
}