use std::fs;

#[derive(Clone, Debug)]
struct Elf {
    inventory: Vec<u32>,
}

impl Elf {

    pub fn addInventory(&mut self, inv: u32) {
        self.inventory.push(inv);
    }

    pub fn totalCalories(&self) -> u32 {
        self.inventory.iter().sum::<u32>()
    }

}
impl std::fmt::Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.inventory.iter().sum::<u32>(), self.inventory.len())
    }
}

fn main() {
    let input_part_1 = fs::read_to_string("./fixtures/input_part_1.txt")
        .expect("could not parse input_part_1.txt from fixtures/ dir.");
    let mut list_of_elves = parse_list(&input_part_1);
    for elf in &list_of_elves {
        print!("{}", elf);
    }
    println!("");

    find_highest_calorie_elf(INPUT_SAMPLE);

    // part 2
    sort_elves_by_calories(&mut list_of_elves);
    let elf_slice = &list_of_elves[0..3];
    for e in elf_slice {
        println!("{}", e.totalCalories());
    }
    let slice_calories = elf_slice.iter().map(|item| item.totalCalories());
    println!("total for top 3 elves: {}", slice_calories.sum::<u32>());
}
fn parse_list(input: &str) -> Vec<Elf> {
    let mut list_of_elves: Vec<Elf> = Vec::new();
    {
        let mut elf: Elf = Elf{
            inventory: Vec::new()
        };
        let v: Vec<&str> =  input.split('\n').collect::<Vec<_>>();
        for word in v {
            if word.trim() == ""  {
                // do we have any inventory started, or is it just a bunch of
                // new lines in a row?
                if elf.inventory.len() > 0 {
                    list_of_elves.push(elf.clone());
                    elf = Elf{
                        inventory: Vec::new()
                    };
                }
            } else {
                let result = word.trim().parse();
                // I hit ParseIntError when i tried to parse > 65535 into u16
                match result {
                    Ok(x)=> elf.addInventory(x),
                    Err(e) => { println!("cannot parse int {}", word.trim()) },
                }
            }
        }
        // if there's an elf with inventory, then the list didn't end in a newline
        // so he's partially parsed (inventory added, but not pushed to list)
        if elf.inventory.len() > 0 {
            list_of_elves.push(elf);
        }
    }
    list_of_elves
}

fn find_highest_calorie_elf(input: &str) {
    let list_of_elves = parse_list(input);
    let mut highest_calorie: u32 = 0;
    let mut highest_index: usize = 0;
    for (index, elf) in list_of_elves.iter().enumerate() {
        if elf.totalCalories() > highest_calorie {
            highest_calorie = elf.totalCalories();
            highest_index = index;
        }
    }
    println!("elf #{} has {} calories", highest_index+1, highest_calorie);
}

fn sort_elves_by_calories(list: &mut Vec<Elf>)
{
    list.sort_by(|item_a, item_b| item_b.totalCalories().cmp(&item_a.totalCalories()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_leading_whitespace_without_adding_extra_elf() {
        let list_of_elves = parse_list("

1000");
        assert_eq!(1, list_of_elves.len());
    }

    #[test]
    fn it_parses_trailing_whitespace_without_adding_extra_elf() {
        let list_of_elves = parse_list("

1000


2000
3000

");
        assert_eq!(2, list_of_elves.len());
    }


    #[test]
    fn it_finds_answer_to_advent() {
        let input_part_1 = fs::read_to_string("./fixtures/input_part_1.txt")
            .expect("could not parse input_part_1.txt from fixtures/ dir.");
        find_highest_calorie_elf(&input_part_1);
    }
}


static INPUT_SAMPLE: &str ="
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
