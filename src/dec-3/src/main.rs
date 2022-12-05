use std::{fs, vec};

#[derive(Debug, PartialEq, Clone)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}
#[derive(Debug, PartialEq)]
struct RucksackThrouple {
    first_sack: Rucksack,
    second_sack: Rucksack,
    third_sack: Rucksack,
}

impl Rucksack {
    /* Methods on Top */

    /// ;)
    fn parse_sacks(input: std::str::Lines<'_>) -> Vec<Rucksack> {
        return input
            .map(|s| Rucksack::parse_sack_contents(&s.to_string()))
            .collect();
    }

    fn parse_sack_contents(values: &String) -> Rucksack {
        let (first, second) = values.trim().split_at(values.len() / 2);
        debug_assert_eq!(first.len(), second.len());

        return Rucksack {
            first_compartment: first.chars().collect(),
            second_compartment: second.chars().collect(),
        };
    }

    /// Usually in a fit of rage
    ///   * Can also marry the compartments without screwing the current separation up
    fn dump_bag(&self) -> Vec<char> {
        self.first_compartment
            .iter()
            .cloned()
            .chain(self.second_compartment.iter().cloned())
            .collect()
    }

    /* Constructor */
    fn new() -> Rucksack {
        return Rucksack {
            first_compartment: vec![],
            second_compartment: vec![],
        };
    }
}

impl RucksackThrouple {
    /* Methods on Top */
    fn parse(input: std::str::Lines<'_>) -> Vec<RucksackThrouple> {
        let mut all_grouped: Vec<RucksackThrouple> = Vec::new();
        let mut all: Vec<Rucksack> = Vec::new();

        let mut three_rucks: Vec<Rucksack> = Vec::new();

        all = Rucksack::parse_sacks(input);
        for chunk in all.chunks(3) {
            three_rucks = chunk.into_iter().map(|r| r.to_owned()).collect::<Vec<_>>();
            all_grouped.push(RucksackThrouple::from_vec(three_rucks))
        }

        all_grouped
    }

    fn find_badge(&self) -> char {
        let first = self.first_sack.dump_bag();
        let second = self.second_sack.dump_bag();
        let third = self.third_sack.dump_bag();

        let binding = common_priorities_throuple(first, second, third);
        let badge = binding.first().unwrap();

        *badge
    }

    /* Accessory + Constructor Associated Functions */
    fn from_vec(v: Vec<Rucksack>) -> RucksackThrouple {
        assert_eq!(v.len(), 3);

        RucksackThrouple {
            first_sack: v.get(0).unwrap().to_owned(),
            second_sack: v.get(1).unwrap().to_owned(),
            third_sack: v.get(2).unwrap().to_owned(),
        }
    }

    fn new() -> RucksackThrouple {
        return RucksackThrouple {
            first_sack: Rucksack::new(),
            second_sack: Rucksack::new(),
            third_sack: Rucksack::new(),
        };
    }
}

fn common_priorities_throuple(a: Vec<char>, b: Vec<char>, c: Vec<char>) -> Vec<char> {
    a.into_iter()
        .filter(|item| b.contains(item) && c.contains(item))
        .collect()
}

// impl ElfSquadron {
//     // The name is just too good to leave
//     fn parse_sacks(input: std::str::Lines<'_>) -> Vec<Rucksack> {
//         return input
//             .enumerate()
//             .map(|s| ElfSquadron::parse_sack_contents(&s.to_string()))
//             .collect();
//     }

//     fn new() -> ElfSquadron {
//         ElfSquadron {
//             first: vec![],
//             second: vec![],
//             third: vec![],
//         }
//     }
// }

// fn load_squadron(i: usize, &mut elf_squadd: RucksackThrouple) -> Option<RucksackThrouple> {
//     match i % 3 {
//         0 => {
//             if elf_squadd.first.len() > 0 {
//                 elf_squadd.push(elf_squadd);
//             }

//             elf_squadd = RucksackThrouple::new();
//             elf_squadd.first = i.chars().collect();
//         }
//         1 => elf_squadd.second = e.chars().collect(),
//         2 => elf_squadd.third = e.chars().collect(),
//         n => eprintln!("impossible int returned from i % 3 here: {}", n),
//     }
// }

// fn parse_squadrons(input: std::str::Lines<'_>) -> Vec<RucksackThrouple> {
//     let mut all_squads: Vec<RucksackThrouple>;
//     let mut elf_squadd = RucksackThrouple::new();

//     input
//         .enumerate()
//         .map(|(i, e)| match i % 3 {
//             0 => {
//                 if elf_squadd.first.len() > 0 {
//                     all_squads.push(elf_squadd);
//                 }

//                 elf_squadd = RucksackThrouple::new();
//                 elf_squadd.first_sack = e.chars().collect();
//             }
//             1 => elf_squadd.second_sack = e.chars().collect(),
//             2 => elf_squadd.third_sack = e.chars().collect(),
//             n => eprintln!("impossible int returned from i % 3 here: {}", n),
//         })
//         .collect();

//     all_squads
// }

fn common_priority(first: &Vec<char>, second: &Vec<char>) -> i32 {
    let mut vec: Vec<char> = first
        .into_iter()
        .filter(|c| second.contains(c))
        .map(|c| *c)
        .collect();

    vec.sort();
    vec.dedup();

    assert!(vec.len() == 1);

    vec[0].priority_of_silly_ass_elf_character()
}

trait Priority {
    fn priority_of_silly_ass_elf_character(&self) -> i32;
}

/// trait allows us to define fns on primitive types (yayyy)
//      book example: impl Float for f32
impl Priority for char {
    fn priority_of_silly_ass_elf_character(&self) -> i32 {
        let char_offset: i32;

        // ASCII table offsets for lower and capital characters
        //   in order to map to their assigned values
        if self.is_lowercase() {
            char_offset = 96;
        } else {
            char_offset = 38
        }

        // Convert the character to an i32 and ensure the offset won't lead to an invalid answer
        let ascii_val = *self as i32;
        debug_assert!(ascii_val > char_offset);

        return ascii_val - char_offset;
    }
}

fn total_priorities(sacks: &Vec<Rucksack>) -> i32 {
    sacks
        .into_iter()
        .map(|s| common_priority(&s.first_compartment, &s.second_compartment))
        .sum()
}

fn main() {
    // let fname = "backpack-vals.txt";
    // let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");
    // let sacks = Rucksack::parse_sacks(text.lines().into_iter());

    let fname = "squad-vals.txt";
    let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");
    let sack_throuples = RucksackThrouple::parse(text.lines().into_iter());

    let priority_sum: i32 = sack_throuples
        .into_iter()
        .map(|thr| thr.find_badge().priority_of_silly_ass_elf_character())
        .sum();

    println!("total: {}", priority_sum);
}
