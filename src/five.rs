use crate::get_puzzle_input;

pub fn five(part: bool) {
	let inp = get_puzzle_input("input/five_input.txt");
	let crates = inp[0].to_owned();
	let mut columns: [Vec<char>; 9] = Default::default();

	let instruct = inp[1].to_owned();
	{
		let mut c_count = 0;
		for c in crates.chars() {
			if c.is_alphabetic() {
				columns[c_count / 4].push(c);
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
	for vec in &mut columns {
		vec.reverse();
	}
	let instruct: Vec<&str> = instruct.split('\n').collect();
	for line in instruct {
		let line_split: Vec<&str> = line.split(' ').collect();
		if line_split.len() < 6 {
			continue;
		}
		let loops: usize = line_split[1].parse().expect("Invalid instruction: Can't move NaN crates");
		let orig: usize = line_split[3].parse().expect("Invalid instruction: Can't move from NaN");
		let dest: usize = line_split[5].trim().parse().expect("Invalid instruction: Can't move to NaN");
		if !part {
			for _ in 0..loops {
				let c = columns[orig - 1].pop().unwrap();
				columns[dest - 1].push(c);
			}
		}
		else {
			let mut  c = columns[orig - 1].split_off(columns[orig - 1].len() - loops);
			columns[dest - 1].append(&mut c);
		}
		
	}
	for v in columns {
		print!("{}", v[v.len()-1]);
	}
	println!();
}