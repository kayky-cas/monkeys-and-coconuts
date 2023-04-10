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

        let mut even_coconuts = 0;
        let mut odd_coconuts = 0;

        for &x in &monkey_info[3..] {
            if x % 2 == 0 {
                even_coconuts += 1;
            } else {
                odd_coconuts += 1;
            }
        }

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

        let monkeys: Vec<_> = lines.flat_map(|line| line.parse()).collect();

        Ok(Self { rounds, monkeys })
    }
}
