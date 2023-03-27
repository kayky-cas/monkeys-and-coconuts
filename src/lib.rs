use std::str::FromStr;

pub struct CoconutGame {
    pub rounds: usize,
    monkeys: Vec<(usize, usize)>,
    coconuts: Vec<(usize, usize)>,
}

impl CoconutGame {
    pub fn play(&mut self) -> (usize, usize) {
        let mut rounds = self.rounds;

        while rounds > 0 {
            rounds -= 1;

            for x in 0..self.monkeys.len() {
                self.coconuts[self.monkeys[x].0].0 += self.coconuts[x].0;
                self.coconuts[x].0 = 0;

                self.coconuts[self.monkeys[x].1].1 += self.coconuts[x].1;
                self.coconuts[x].1 = 0;
            }
        }

        return (0..self.monkeys.len())
            .map(|x| (x, self.coconuts[x].0 + self.coconuts[x].1))
            .max_by(|curr, oth| curr.1.cmp(&oth.1))
            .unwrap();
    }
}

impl FromStr for CoconutGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds = s
            .lines()
            .nth(0)
            .unwrap()
            .split(' ')
            .map(|s| s.parse().ok())
            .flatten()
            .nth(0)
            .unwrap();

        let monkeys_arr: Vec<_> = s
            .lines()
            .skip(1)
            .map(|line| {
                line.split(' ')
                    .map(|x| x.parse::<usize>().ok())
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut monkeys = Vec::new();
        let mut coconuts = Vec::new();

        for monkey in monkeys_arr {
            monkeys.push((monkey[1], monkey[2]));

            let mut odd_c = 0;
            let mut even_c = 0;

            for coconut in &monkey[2..] {
                if *coconut % 2 == 0 {
                    even_c += 1;
                } else {
                    odd_c += 1;
                }
            }

            coconuts.push((even_c, odd_c));
        }

        Ok(Self {
            rounds,
            monkeys,
            coconuts,
        })
    }
}
