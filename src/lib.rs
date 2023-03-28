use std::str::FromStr;

struct Monkey {
    even: usize,
    odd: usize,
    even_coconuts: usize,
    odd_coconuts: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkey_info = s
            .split(' ')
            .map(|x| x.parse::<usize>())
            .flatten()
            .skip(1)
            .collect::<Vec<_>>();

        let even = monkey_info[0];
        let odd = monkey_info[1];

        let coconuts = &monkey_info[2..];

        let even_coconuts = coconuts.iter().filter(|&x| x % 2 == 0).count();
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

            for x in 0..self.monkeys.len() {
                let even = self.monkeys[x].even;
                self.monkeys[even].even_coconuts += self.monkeys[x].even_coconuts;
                self.monkeys[x].even_coconuts = 0;

                let odd = self.monkeys[x].odd;
                self.monkeys[odd].odd_coconuts += self.monkeys[x].odd_coconuts;
                self.monkeys[x].odd_coconuts = 0;
            }
        }

        return (0..self.monkeys.len())
            .map(|x| {
                (
                    x,
                    self.monkeys[x].odd_coconuts + self.monkeys[x].even_coconuts,
                )
            })
            .max_by(|curr, oth| curr.1.cmp(&oth.1))
            .unwrap();
    }
}

impl FromStr for CoconutGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let rounds = lines
            .nth(0)
            .unwrap()
            .split(' ')
            .map(|s| s.parse())
            .flatten()
            .nth(0)
            .unwrap();

        let monkeys = lines.map(|l| l.parse()).flatten().collect();

        Ok(Self { rounds, monkeys })
    }
}
