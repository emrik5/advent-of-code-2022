use crate::get_puzzle_input;

pub fn five() {
	let inp = get_puzzle_input("input/five_input.txt");
	let crates = inp[0].to_owned();
	let mut rows: [Vec<bool>; 9] = Default::default();

	let instruct = inp[1].to_owned();
	{
		let mut spaces_count = 0;
		for c in crates.chars() {
			if c == ' ' {
				spaces_count += 1;
			}
			else if c == '[' {
				rows[spaces_count % 4].push(true);
				spaces_count = 0;
			}
		}
	}
	println!("{:?}", rows)
}