use std::{fs, cmp::Ordering};

#[derive(Clone)]
struct Rucksack {
    compartment_a: String,
    compartment_b: String,
}
mod part2;

fn main() {
    if false {
        part_one();
    } else {
        part2::part_two();
    }
}

fn part_one() {
    // let _input_sample = fs::read_to_string("./fixtures/sample.txt")
    //    .expect("could not parse sample.txt from fixtures/ dir.");

    let _input_sample = fs::read_to_string("./fixtures/part_1_input.txt")
       .expect("could not parse part_1_input.txt from fixtures/ dir.");

    let ruck_sack_vec = parse_into_rucksacks(&_input_sample);
    for sack in &ruck_sack_vec{
        println!("A: {} B: {}", sack.compartment_a, sack.compartment_b);
    }

    let mut priority_score:i32 = 0;
    for sack in ruck_sack_vec {
        let ret = find_similar_types_in_compartments(&sack);
        priority_score += match ret {
            // Some(x) => x as i32 - 96,
            Some(x) => lookup_priority(&x),
            None => 0 as i32,
        };
    }
    println!("Found priority sum of {}", priority_score);
}

fn parse_into_rucksacks(input: &str) -> Vec<Rucksack> {

    let mut matches: Vec<Rucksack> = Vec::new();

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

        matches.push(
            Rucksack{
                compartment_a: String::from_iter(chars_a),
                compartment_b: String::from_iter(chars_b),
            }
        )
    }
    matches
}


// if matchin b is greater than matchin a, then we can break
// the search, because everything is sorted.
// i think this takes it down from O(n^2) to O(log n)
fn find_similar_types_in_compartments(sack: &Rucksack) -> Option<char>
{
    for item_type_b in sack.compartment_b.chars() {
        for item_type in sack.compartment_a.chars() {
            let ordering = custom_type_priority_sorting(&item_type, &item_type_b);
            let found_duplicate = match ordering {
                Ordering::Equal => true,
                Ordering::Less => false,
                Ordering::Greater => break,
            };
            if found_duplicate {
                return Some(item_type.to_owned())
            }
        }
    }
    None
}

fn custom_type_priority_sorting(a: &char, b: &char) -> std::cmp::Ordering {
    if b.is_uppercase() && a.is_lowercase() {
        return Ordering::Less;
    }
    if a.is_uppercase() && b.is_lowercase() {
        return Ordering::Greater;
    }
    a.cmp(b)
}

fn lookup_priority(c: &char) -> i32 {

    if c.is_lowercase() {
        return c.to_owned() as i32 - 96;
    } 
    c.to_owned() as i32 - 38
}