use std::{fs, cmp::Ordering};

use super::{Rucksack, custom_type_priority_sorting};

const SIZE_OF_GROUPS: usize = 3;

struct StringComparator<'a> {
    currenta: usize,
    currentb: usize,
    sak_a: &'a Rucksack,
    sak_b: &'a Rucksack,
}
impl<'a> StringComparator<'a> {
    fn new (sak_a: &'a Rucksack, sak_b: &'a Rucksack) -> StringComparator<'a> {
        StringComparator {
            currenta: 0,
            currentb: 0,
            sak_a: sak_a,
            sak_b: sak_b,
        }
    }
}
impl<'a> Iterator for StringComparator<'a> {
    type Item = char;

    /**
     * this function is not stateful as of yet
     *
     * rs (bgllqGHHLPPRRTZZ, fffjnnrttvvvvzJZ)
     * rs (ccchhhswCDDNVV, ffjjnvvzzFJTVW)
     * rs (cmpCCMNT, mqBGGHLL)
     * sack 1 and 2 have in common: f
     * sack 1 and 2 have in common: f
     * ---
     */
    fn next(&mut self) -> Option<char> {
        // this should create slices of combined compartent_a and compartment_b
        // and start iterating at the currenta or currentb slice element.
        // this function is not stateful as of yet

        if self.currenta > self.sak_a.compartment_a.len() + self.sak_a.compartment_b.len() {
            return None;
        };
        if self.currentb > self.sak_b.compartment_a.len() + self.sak_b.compartment_b.len(){
            return None;
        };
        for item_type_b in self.sak_b.compartment_a.chars() {
            self.currentb += 1;
            for item_type in self.sak_a.compartment_a.chars() {
                self.currenta += 1;
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
            for item_type in self.sak_a.compartment_b.chars() {
                self.currenta += 1;
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
        self.currenta = 0;
        for item_type_b in self.sak_b.compartment_b.chars() {
            self.currentb += 1;
            for item_type in self.sak_a.compartment_a.chars() {
                self.currenta += 1;
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
            for item_type in self.sak_a.compartment_b.chars() {
                self.currenta += 1;
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
}

pub fn part_two() {
    let _input_part_2 = fs::read_to_string("./fixtures/part_2_input.txt")
       .expect("could not parse part_2_input.txt from fixtures/ dir.");

    let _group_of_rucksacks = parse_into_rucksacks(&_input_part_2);

    find_common_item_in_groups(_group_of_rucksacks);
}

fn find_common_item_in_groups(rucksack_arrays: Vec<[Rucksack; SIZE_OF_GROUPS]>)
{
    for rucksak_array in rucksack_arrays {
        println!("rs {}", rucksak_array[0]);
        println!("rs {}", rucksak_array[1]);
        println!("rs {}", rucksak_array[2]);
        let mut comparator = StringComparator::new(&rucksak_array[0], &rucksak_array[1]);
        let mut common;
        let mut end:bool = false;
        while !end {
            common = comparator.next() ;
            match common {
                Some(x) => println!("sack 1 and 2 have in common: {}", x),
                None => end = true,
            }
        }
        println!("---");
    }
}

fn find_similar_types_in_two_sacks(sack_a: &Rucksack, sack_b: &Rucksack) -> Option<char>
{
    for item_type_b in sack_b.compartment_a.chars() {
        for item_type in sack_a.compartment_a.chars() {
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
        for item_type in sack_a.compartment_b.chars() {
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
    for item_type_b in sack_b.compartment_b.chars() {
        for item_type in sack_a.compartment_a.chars() {
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
        for item_type in sack_a.compartment_b.chars() {
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



fn parse_into_rucksacks(input: &str) -> Vec<[Rucksack; SIZE_OF_GROUPS]> {

    let mut matches: Vec<[Rucksack; SIZE_OF_GROUPS]> = Vec::new();

    let rsack_a = Rucksack{compartment_a: "".to_string(), compartment_b: "".to_string()};
    let rsack_b = Rucksack{compartment_a: "".to_string(), compartment_b: "".to_string()};
    let rsack_c = Rucksack{compartment_a: "".to_string(), compartment_b: "".to_string()};

    let v: Vec<&str> =  input.split('\n').collect::<Vec<_>>();
    let mut rsack_index = 0;
    let mut current_array: [Rucksack; 3] = [ rsack_a.clone(), rsack_b.clone(), rsack_c.clone() ];
	for letters in v {
		if letters.len() == 0 {
			break
		}
        if rsack_index == 3 {
            rsack_index = 0;
            matches.push(current_array.clone());
            current_array = [ rsack_a.clone(), rsack_b.clone(), rsack_c.clone() ];
        }

        let(first, last) = letters.split_at( letters.len() / 2 );

        let mut chars_a: Vec<char> = first.chars().collect();
        // default rust sort puts uppercase before lowercase
        chars_a.sort_by(|a, b| custom_type_priority_sorting(a, b));

        let mut chars_b: Vec<char> = last.chars().collect();
        // default rust sort puts uppercase before lowercase
        chars_b.sort_by(|a, b| custom_type_priority_sorting(a, b));

        let rsack_a = Rucksack {
            compartment_a: String::from_iter(chars_a),
            compartment_b: String::from_iter(chars_b),
        };
        current_array[rsack_index] = *Box::new(rsack_a);
        rsack_index += 1;
    }
    matches
}

