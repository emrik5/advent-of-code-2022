use std::error::Error;
use std::fs;
use std::io;

pub mod five;
pub mod seven;
fn main() -> Result<(), Box<dyn Error>> {
        println!("Enter Day");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp)?;
        let inp: u32 = inp.trim().parse().unwrap_or(0);

        println!("Enter Part (1 or 2)");
        let mut part = String::new();
        io::stdin().read_line(&mut part)?;
        let part: u32 = part.trim().parse()?;
        let part: bool = match part {
            1 => false,
            2 => true,
            _ => {
                println!("Invalid Part, defaults to 1");
                false
                }
        };

        match inp {
            0 => println!("No or invalid input"),
            1 => todo!(),
            2 => todo!(),
            3 => todo!(),
            4 => todo!(),
            5 => five::five(part),
            6 => todo!(),
            7 => seven::seven(part),
            _ => unreachable!()
        }
    Ok(())
}

pub fn get_puzzle_input(file_path: &str) -> Vec<String> {
    let inp = fs::read_to_string(file_path).expect("Failed to read puzzle input file");
    let mut outp: Vec<String> = vec![];
    let inp: Vec<&str> = inp.split('^').collect();
    for c in inp {
        outp.push(c.to_string());
    }
    outp
    
}