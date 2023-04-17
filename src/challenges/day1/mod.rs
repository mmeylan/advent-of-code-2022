pub fn run() {
    println!("Day 1 running...")
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

fn find_next_meal(input: &str) -> i32 {
    parse_elf_input(input).iter().fold(0, |calories, elf| {
        if elf.carried_calories > calories {
            elf.carried_calories
        } else {
            calories
        }
    })
}

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
    use crate::challenges::day1::find_next_meal;

    #[test]
    fn dumb_test() {
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

        let most = find_next_meal(input);
        assert_eq!(most, 24000);
    }
}