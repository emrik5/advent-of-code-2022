use core::panic;

use crate::get_puzzle_input;
enum ItemType {
    Dir,
    File
}
struct Item<'a> {
    typ: ItemType,
    parent: &'a Item<'a>,
    children: Vec<&'a Item<'a>>,
}

pub fn seven(part: bool) {
    let inp = get_puzzle_input("input/seven_input.txt");
    let inp: Vec<&str> = inp[0].split('\n').collect();
    let mut current_item: &Item = &Item { typ: ItemType::Dir, parent: , children: Vec::new() };
    for line in inp {
        let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
        match split_line[0] {
            "$" => match split_line[1] {
                "ls" => {},
                "cd" => current_item = match split_line[2] {
                    ".." => current_item.parent,
                    _ => panic!(),
                },
                _ => panic!()
            },
            "dir" => {
                
            },
            _ => panic!()
        }
    }
    
}