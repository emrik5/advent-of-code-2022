use crate::get_puzzle_input;

pub fn five() {
	let inp = get_puzzle_input("input/five_input.txt");
	let crates = inp[0].to_owned();
	let mut rows: [Vec<bool>; 9] = Default::default();

	let instruct = inp[1].to_owned();
    println!("{:?}", crates);
	{
		let mut c_count = 0;
		for c in crates.chars() {
			if c == '[' {
				rows[c_count / 4].push(true);
                c_count += 1;
			}
            else if c == '\n' || c == '\r' {
                c_count = 0;
            }
            else {
                c_count += 1;
            }
		}
	}
	println!("{:?}", rows)
}