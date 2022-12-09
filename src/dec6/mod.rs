use std::{collections::VecDeque, fs};

type Signal = Vec<char>;

trait ISig {
    fn parse(input: String) -> Signal;
    fn marker_pos(&self, required_distinct: usize) -> Option<i32>;
}

impl ISig for Signal {
    fn parse(input: String) -> Signal {
        let s: Signal = input.chars().collect();
        s
    }

    fn marker_pos(&self, required_distinct: usize) -> Option<i32> {
        // offset is the index at which a match can reasonably be found at
        let offset: usize = (required_distinct - 1) as usize;

        // In case we're given a string that's smaller than the offset requirement
        if self.len() < offset {
            return None;
        }

        for current in offset..self.len() {
            if current < offset + 1 {
                continue;
            }

            // Create a copy of each index and the preceding chars that may be a marker
            let mut marker_candidate = self[current - required_distinct..current].to_owned();
            marker_candidate.sort();

            // Sort and deduplicate them to evaluate their lengths
            marker_candidate.dedup();

            // If the lengths are the same, all identified characters are unique
            if self[current - required_distinct..=current].len() == marker_candidate.len() {
                return Some(current as i32);
            }
        }

        None
    }
}

pub fn _part_one() {
    let fname = "src/dec6/comms-stream.txt";
    let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");

    for line in text.lines() {
        let input = Signal::parse(line.to_string()) as Signal;
        let idx = input.marker_pos(4);

        println!("Index of packet start: {}", idx.unwrap());
    }
}

pub fn part_two() {
    let fname = "src/dec6/comms-stream.txt";
    let text = fs::read_to_string(fname.to_string()).expect("failed to read to string");

    for line in text.lines() {
        let input = Signal::parse(line.to_string()) as Signal;
        let idx = input.marker_pos(14);

        println!("Index of message start: {}", idx.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::{ISig, Signal};

    #[test]
    fn test_message_start_marker() {
        let msg1 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let sig1 = Signal::parse(msg1) as Signal;
        let idx = sig1.marker_pos(14).unwrap();

        assert_eq!(idx, 26);

        let msg2 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let sig2 = Signal::parse(msg2) as Signal;
        let idx2 = sig2.marker_pos(14);

        assert_eq!(idx2.unwrap(), 19);
    }
}
