use std::{fs, cmp::Ordering};

use super::{Rucksack, custom_type_priority_sorting};

pub fn part_two() {
    let _input_part_2 = fs::read_to_string("./fixtures/part_2_input.txt")
       .expect("could not parse part_2_input.txt from fixtures/ dir.");

    let groupOfRucksacks = parse_into_rucksacks(&_input_part_2, 3);
}

fn parse_into_rucksacks(input: &str, size_of_groups: usize) -> Vec<[Rucksack; 3]> {

    let mut matches: Vec<[Rucksack; 3]> = Vec::new();

    // let rsack_a = Rucksack{compartment_a: "".to_string(), compartment_b: "".to_string()};
    let rsack_b = Rucksack{compartment_a: "".to_string(), compartment_b: "".to_string()};
    let rsack_c = Rucksack{compartment_a: "".to_string(), compartment_b: "".to_string()};

    let v: Vec<&str> =  input.split('\n').collect::<Vec<_>>();
	for letters in v {
		if letters.len() == 0 {
			break
		}
        let(first, last) = letters.split_at( letters.len() / 2 );
        let mut chars_a: Vec<char> = first.chars().collect();
        // default rust sort puts uppercase before lowercase
        chars_a.sort_by(|a: &char, b: &char| {
            if b.is_uppercase() && a.is_lowercase() {
                return Ordering::Less;
            }
            if a.is_uppercase() && b.is_lowercase() {
                return Ordering::Greater;
            }

            a.cmp(b)
        });
        let mut chars_b: Vec<char> = last.chars().collect();
        chars_b.sort_by(|a, b| custom_type_priority_sorting(a, b));

        let rsack_a = Rucksack{
            compartment_a: String::from_iter(chars_a),
            compartment_b: String::from_iter(chars_b),
        };
        matches.push(
            [
                rsack_a.clone(),
                rsack_b.clone(),
                rsack_c.clone()
            ]
        )
    }
    matches
}

