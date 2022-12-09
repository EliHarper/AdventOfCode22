/*

Supply Stacks

- Supplies are in stacks marked crates
- Ship can move them with a cargo crane

- Crates are visualized like this:

       [D]  [E]
  [A] [B] [C]

Instructions: find crates on top after rearrangement

*/

use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

type ShippingCrate = char;
type CrateStacks = HashMap<i32, Vec<ShippingCrate>>;

trait ICrateStack {
    fn add_below(&mut self, sc: ShippingCrate, num: i32);
}

impl ICrateStack for CrateStacks {
    fn add_below(&mut self, sc: ShippingCrate, num: i32) {
        let mut new_crate_vec = vec![sc];

        match self.entry(num) {
            Entry::Occupied(mut e) => {
                let mut stack = e.get();
                new_crate_vec.extend(stack.iter().cloned());

                // self.insert(num, new_crate_vec);
                &mut e.insert(new_crate_vec)
            }
            Entry::Vacant(e) => e.insert(vec![sc]),
        };
    }
}

trait Stack {}

impl Stack for CrateStacks {}

struct Move {
    quantity: i32,
    from: i32,
    to: i32,
}

impl Move {}

fn init_stack_line(line: String, cs: &mut CrateStacks) {
    let mut column = 0;
    let crate_chars: Vec<char> = line.chars().filter(|s| s.is_alphabetic()).collect();

    for c in crate_chars {
        // Position in line will indicate the column
        let position = line.find(c).unwrap() as i32;
        if position > 4 {
            // With an offset of 1, characters are 4 apart
            column = (position - 1) / 4;
        }

        // Because the input column number isn't an index
        column += 1;

        cs.add_below(c, column);
    }
}

pub fn part_one() {
    let fname = "src/dec4/elf-sections.txt";
    let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");

    let mut cs = CrateStacks::new();

    // While line contains "[" or "]".. init_stack_line
}

#[cfg(test)]
mod tests {
    use super::{init_stack_line, CrateStacks, Move, ShippingCrate, Stack};

    #[test]
    fn test_init_stack_line() {
        let mut cs = CrateStacks::new();
        let line = "        [J]         [B]     [T]    ";

        init_stack_line(line.to_string(), &mut cs);

        assert_eq!(cs.get(&3).unwrap().to_owned(), vec!['J']);
        assert_eq!(cs.get(&6).unwrap().to_owned(), vec!['B']);
        assert_eq!(cs.get(&8).unwrap().to_owned(), vec!['T']);

        init_stack_line("        [M] [L]     [Q] [L] [R]    ".to_string(), &mut cs);

        assert_eq!(cs.get(&3).unwrap().to_owned(), vec!['M', 'J']);
        assert_eq!(cs.get(&6).unwrap().to_owned(), vec!['Q', 'B']);
    }

    // #[test]
    //     fn test_init_all_stack_lines() {
    //         let mut cs = CrateStacks::new();
    //         let all_stacks = r#"        [J]         [B]     [T]
    //         [M] [L]     [Q] [L] [R]
    //         [G] [Q]     [W] [S] [B] [L]
    // [D]     [D] [T]     [M] [G] [V] [P]
    // [T]     [N] [N] [N] [D] [J] [G] [N]
    // [W] [H] [H] [S] [C] [N] [R] [W] [D]
    // [N] [P] [P] [W] [H] [H] [B] [N] [G]
    // [L] [C] [W] [C] [P] [T] [M] [Z] [W]
    // "#;

    //         for line in all_stacks.lines() {
    //             init_stack_line(line.to_string(), &mut cs)
    //         }

    //         println!("{:?}", cs);
    //         assert_eq!(cs.get(&2).unwrap().to_owned(), vec!['C', 'P', 'H']);
    //     }
}
