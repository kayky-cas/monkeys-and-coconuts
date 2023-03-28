use std::str::FromStr;

struct Monkey {
    even: usize,
    odd: usize,
    even_coconuts: usize,
    odd_coconuts: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(buffer: &str) -> Result<Self, Self::Err> {
        let monkey_info: Vec<_> = buffer
            .split(' ')
            .flat_map(|word| word.parse())
            .skip(1)
            .collect();

        let even = monkey_info[0];
        let odd = monkey_info[1];

        let coconuts = &monkey_info[3..];

        let even_coconuts = coconuts
            .iter()
            .filter(|&coconut| coconut & 0b1 == 0)
            .count();

        let odd_coconuts = coconuts.len() - even_coconuts;

        Ok(Self {
            even,
            odd,
            even_coconuts,
            odd_coconuts,
        })
    }
}

pub struct CoconutGame {
    pub rounds: usize,
    monkeys: Vec<Monkey>,
}

impl CoconutGame {
    pub fn play(&mut self) -> (usize, usize) {
        let mut rounds = self.rounds;

        while rounds > 0 {
            rounds -= 1;

            for index in 0..self.monkeys.len() {
                let even_index = self.monkeys[index].even;
                self.monkeys[even_index].even_coconuts += self.monkeys[index].even_coconuts;
                self.monkeys[index].even_coconuts = 0;

                let odd_index = self.monkeys[index].odd;
                self.monkeys[odd_index].odd_coconuts += self.monkeys[index].odd_coconuts;
                self.monkeys[index].odd_coconuts = 0;
            }
        }

        return self
            .monkeys
            .iter()
            .enumerate()
            .map(|(index, monkey)| (index, monkey.odd_coconuts + monkey.even_coconuts))
            .max_by(|curr, oth| curr.1.cmp(&oth.1))
            .unwrap();
    }
}

impl FromStr for CoconutGame {
    type Err = ();

    fn from_str(buffer: &str) -> Result<Self, Self::Err> {
        let mut lines = buffer.lines();

        let rounds = lines
            .nth(0)
            .unwrap()
            .split(' ')
            .flat_map(|word| word.parse())
            .nth(0)
            .unwrap();

        let monkeys: Vec<_> = lines.map(|line| line.parse()).flatten().collect();

        if monkeys.len() < 3 {
            Err(())
        } else {
            Ok(Self { rounds, monkeys })
        }
    }
}
