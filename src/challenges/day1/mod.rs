use crate::challenges::utils::read_file;

pub fn run() {
    let input = read_file("src/challenges/day1/input.txt");
    let top_elf = find_top_carrier(input.as_str());
    println!("Found {} calories in the richest elf", top_elf.carried_calories);

    let top_elves = find_top_n_carriers(input.as_str(), 3);
    let total = top_elves.iter().fold(0, |agg, x| agg + x.carried_calories);
    println!("Found {} calories in the richest elves in phase 2", total);
}

fn parse_elf_input(input: &str) -> Vec<Elf> {
    let data = String::from(input);
    let mut elves: Vec<Elf> = vec!(Elf::new());
    for token in data.split("\n") {
        if token.len() == 0 {
            elves.push(Elf::new())
        } else {
            let current_elf: &mut Elf = elves.last_mut().unwrap();
            // let mut current_elf = Elf::new();
            // current_elf.add_calories(20)
            match token.parse::<i32>() {
                Ok(calories) => current_elf.add_calories(calories),
                Err(e) => panic!("{}: {}", e, token)
            }
        }
    }
    elves
}

fn find_top_carrier(input: &str) -> Elf {
    find_top_n_carriers(input, 1).first().unwrap().clone()
}

fn find_top_n_carriers(input: &str, n: usize) -> Vec<Elf> {
    let mut elves = parse_elf_input(input);
    elves.sort_by(|a, b| a.carried_calories.cmp(&b.carried_calories).reverse());
    elves[..=n - 1].to_vec()
}

#[derive(Clone)]
struct Elf {
    carried_calories: i32,
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            carried_calories: 0
        }
    }

    pub fn add_calories(&mut self, calories: i32) {
        self.carried_calories += calories
    }
}


#[cfg(test)]
mod day1_tests {
    use crate::challenges::day1::{find_top_carrier, find_top_n_carriers};

    #[test]
    fn phase_1_example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let most = find_top_carrier(input);
        assert_eq!(most.carried_calories, 24000);
    }

    #[test]
    fn phase_2_example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let top_3 = find_top_n_carriers(input, 3);
        println!("top1 elf {}", top_3.get(0).unwrap().carried_calories);
        println!("top2 elf {}", top_3.get(1).unwrap().carried_calories);
        println!("top3 elf {}", top_3.get(2).unwrap().carried_calories);
        println!("num elf {}", top_3.len());
        assert_eq!(top_3.iter().fold(0, |agg, x| agg + x.carried_calories), 45000);
    }
}