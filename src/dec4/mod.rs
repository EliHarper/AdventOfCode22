/* Camp Cleanup */

use std::{fs, str::Split};

#[derive(Debug)]
struct ConsecutiveSections {
    beginning: i32,
    end: i32,
}

/// A tad reductionist; poor elves
#[derive(Debug)]
struct ElfPair {
    elf_one: ConsecutiveSections, // Always gets the lowest "beginning" number in range
    elf_two: ConsecutiveSections,
}

impl ElfPair {
    fn have_contained_section(&self) -> bool {
        return self.elf_one.end >= self.elf_two.end;
    }

    fn overlapping(&self) -> bool {
        let one_range = self.elf_one.beginning..=self.elf_one.end;
        one_range.contains(&self.elf_two.beginning)
    }

    /// Create a pair with order enforced, using an unordered pair
    ///   - If I cared enough, I'd make this a sort function lol
    fn new_ordered(u: &ElfPair) -> ElfPair {
        let a_first = ElfPair {
            elf_one: ConsecutiveSections {
                beginning: u.elf_one.beginning,
                end: u.elf_one.end,
            },
            elf_two: ConsecutiveSections {
                beginning: u.elf_two.beginning,
                end: u.elf_two.end,
            },
        };

        let b_first = ElfPair {
            elf_one: ConsecutiveSections {
                beginning: u.elf_two.beginning,
                end: u.elf_two.end,
            },
            elf_two: ConsecutiveSections {
                beginning: u.elf_one.beginning,
                end: u.elf_one.end,
            },
        };

        if u.elf_one.beginning < u.elf_two.beginning {
            return a_first;
        }

        if u.elf_one.beginning == u.elf_two.beginning {
            // If range beggining is equal, return the one w/ lower end section first
            //   since comparison is of #1's end value being gte #2's end value
            if u.elf_one.end >= u.elf_two.end {
                return a_first;
            } else {
                return b_first;
            }
        }

        b_first
    }

    fn new_empty() -> ElfPair {
        return ElfPair {
            elf_one: ConsecutiveSections {
                beginning: 0,
                end: 0,
            },
            elf_two: ConsecutiveSections {
                beginning: 0,
                end: 0,
            },
        };
    }
}

fn assign_section_extrema(sections: &mut ConsecutiveSections, min_max: Split<char>) {
    min_max.enumerate().for_each(|(i, extrema)| {
        if i < 1 {
            sections.beginning = extrema.parse::<i32>().unwrap();
            return;
        }

        sections.end = extrema.parse::<i32>().unwrap();

        // Ensure we're not looking at more values than expected in the provided range
        assert_eq!(i, 1);
    });
}

fn assign_pairs_extrema(idx: usize, pair: &mut ElfPair, min_max: Split<char>) {
    if idx < 1 {
        // Assign elf_one values
        assign_section_extrema(&mut pair.elf_one, min_max);

        return;
    }

    // elf_two assignment loop
    assign_section_extrema(&mut pair.elf_two, min_max);
}

/// Parse comma-separation and hyphen-denoted range to an ElfPair struct
///   with extrema and ordered elves within the pair
fn parse_pair(line: String) -> ElfPair {
    let mut pair = ElfPair::new_empty();
    let elf_ranges = line.split(",");

    for (idx, range) in elf_ranges.into_iter().enumerate() {
        let min_max = range.split('-');
        assign_pairs_extrema(idx, &mut pair, min_max);
    }

    ElfPair::new_ordered(&pair)
}

pub fn _part_one() {
    let fname = "src/dec4/elf-sections.txt";
    let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");

    let mut contained_ranges = 0;
    for line in text.lines() {
        let elves = parse_pair(line.to_string());

        if elves.have_contained_section() {
            println!(
                "Containing each other: {}-{}, {}-{}",
                elves.elf_one.beginning,
                elves.elf_one.end,
                elves.elf_two.beginning,
                elves.elf_two.end
            );
            contained_ranges += 1;
        }
    }

    println!(
        "{} assignment pairs fully contained each other.",
        contained_ranges
    );
}

pub fn part_two() {
    let fname = "src/dec4/elf-sections.txt";
    let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");

    let mut overlapping = 0;
    for line in text.lines() {
        let elves = parse_pair(line.to_string());

        if elves.overlapping() {
            overlapping += 1;
        }
    }

    println!("{} pairs have ranges that are overlapping", overlapping);
}
