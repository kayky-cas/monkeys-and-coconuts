use std::{cell::RefCell, rc::Rc, str::FromStr};

pub enum CoconutType {
    Even,
    Odd,
}

#[derive(Debug)]
pub struct Monkey {
    even: usize,
    odd: usize,
    evens: usize,
    odds: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = s
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok())
            .flatten()
            .skip(1)
            .collect();

        let even = numbers[0];
        let odd = numbers[1];

        let coconuts = &numbers[2..];
        let evens = coconuts.iter().filter(|&x| x % 2 == 0).count();
        let odds = coconuts.len() - evens;

        Ok(Self {
            even,
            odd,
            evens,
            odds,
        })
    }
}

type MonkeyRef = Rc<RefCell<Monkey>>;

pub struct CoconutGame {
    pub rounds: i32,
    monkeys: Vec<MonkeyRef>,
}

impl CoconutGame {
    fn pass_coconuts(&self, monkey: MonkeyRef, other: MonkeyRef, coconut_type: CoconutType) {
        match coconut_type {
            CoconutType::Even => {
                other.borrow_mut().evens += monkey.borrow().evens;
                monkey.borrow_mut().evens = 0;
            }
            CoconutType::Odd => {
                other.borrow_mut().odds += monkey.borrow().odds;
                monkey.borrow_mut().odds = 0;
            }
        }
    }

    pub fn play(&mut self) -> (usize, usize) {
        let mut rounds = self.rounds;

        while rounds > 0 {
            rounds -= 1;

            for monkey in &self.monkeys {
                let even_monkey = self.monkeys[monkey.borrow().even].clone();
                let odd_monkey = self.monkeys[monkey.borrow().odd].clone();

                self.pass_coconuts(monkey.clone(), even_monkey, CoconutType::Even);
                self.pass_coconuts(monkey.clone(), odd_monkey, CoconutType::Even);
            }
        }

        let winner = self
            .monkeys
            .iter()
            .map(|m| m.borrow().odds + m.borrow().evens)
            .enumerate()
            .max_by(|curr, other| curr.1.cmp(&other.1))
            .unwrap();

        return winner;
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
            .map(|s| s.parse::<i32>().ok())
            .flatten()
            .nth(0)
            .unwrap();

        let monkeys: Vec<_> = s
            .lines()
            .skip(1)
            .map(|line| line.parse::<Monkey>().ok())
            .flatten()
            .map(|monkey| Rc::new(RefCell::new(monkey)))
            .collect();

        if monkeys.len() < 2 {
            return Err(());
        }

        Ok(Self { rounds, monkeys })
    }
}
