use std::collections::HashMap;
use std::fs;
use thiserror::Error; // Great for simplifying Error creation while still allowing variants to be handled differently

/* Error Declaration */
type Result<T> = std::result::Result<T, InvalidIntError>;

#[derive(Error, Debug, Clone)]
#[error("{message:} ({line:}, {column})")]
pub struct InvalidIntError {
    message: String,
    line: usize,
    column: usize,
}
/* End Error Declaration */

struct RoshamboDecoder {
    shape_values: HashMap<String, i32>,
}

impl RoshamboDecoder {
    const LOSS: i32 = 0;
    const DRAW: i32 = 3;
    const WIN: i32 = 6;

    fn configure(&mut self) {
        // Build the ShapeValues hashmap for calculating the score from the chosen shape
        self.shape_values = HashMap::new();

        // Rock
        self.shape_values.insert("A".to_string(), 1);
        self.shape_values.insert("X".to_string(), 1); // Need to lose

        // Paper
        self.shape_values.insert("B".to_string(), 2);
        self.shape_values.insert("Y".to_string(), 2); // Need to end in draw

        // Scissors
        self.shape_values.insert("C".to_string(), 3);
        self.shape_values.insert("Z".to_string(), 3); // Need to win
    }

    fn value_of_round(&self, combo: &str) -> i32 {
        // Split values and make a tuple from them
        let mut our_choices = combo.split_whitespace();
        let tup = (our_choices.next().unwrap(), our_choices.next().unwrap());

        // Get each player's shape value
        let yours = self.get_shape_value(tup.0.to_string());
        let mine = self.get_shape_value(tup.1.to_string()); // Won't be used as shape value in challenge 2

        self.score_with_second_col_as_outcome(mine, yours)

        /* Challenge #1 */
        // self._score_with_second_column_as_shape(mine, yours)
    }

    fn score_with_second_col_as_outcome(&self, mine: i32, yours: i32) -> i32 {
        // Draw
        if mine == 2 {
            return RoshamboDecoder::DRAW + yours;
        }

        let win = mine == 3;
        let shape_value = self.shape_value_from_ltw(win, yours).unwrap();

        if win {
            return shape_value + RoshamboDecoder::WIN;
        }

        shape_value + RoshamboDecoder::LOSS
    }

    // Function used to solve challenge #1
    fn _score_with_second_col_as_shape(&self, mine: i32, yours: i32) -> i32 {
        if mine == yours {
            return RoshamboDecoder::DRAW + mine; // Tie
        }

        // r > s, s > p, p > r
        if (mine == 1 && yours == 3) || (mine == 3 && yours == 2) || (mine == 2 && yours == 1) {
            return RoshamboDecoder::WIN + mine; // Win
        }

        RoshamboDecoder::LOSS + mine //Loss
    }

    fn shape_value_from_ltw(&self, win: bool, yours: i32) -> Result<i32> {
        // r > s, s > p, p > r
        if win {
            return match yours {
                1 => Ok(2), // Winner v. rock: paper
                2 => Ok(3), // Winner v. paper: scissors
                3 => Ok(1), // Winner v. scissors: rock
                v => Err(InvalidIntError {
                    message: format!("Unexpected int value: {}", v),
                    line: line!() as usize,
                    column: column!() as usize,
                }),
            };
        }

        match yours {
            1 => Ok(3),
            2 => Ok(1),
            3 => Ok(2),
            v => Err(InvalidIntError {
                message: format!("Unexpected int value: {}", v),
                line: line!() as usize,
                column: column!() as usize,
            }),
        }
    }

    fn get_shape_value(&self, choice: String) -> i32 {
        self.shape_values[&choice]
    }
}
fn main() {
    let text =
        fs::read_to_string("roshambo-vals.txt".to_string()).expect("failed to read to string");

    // Set up our decoder
    let mut decoder = RoshamboDecoder {
        shape_values: HashMap::new(),
    };
    decoder.configure();

    // Begin tabulating scores:
    let mut total_score = 0;
    for line in text.lines() {
        let line = line.trim();
        total_score += decoder.value_of_round(line);
    }

    println!("My total score is {}", total_score)
}
