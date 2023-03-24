use std::{cell::RefCell, collections::VecDeque, fs, rc::Rc, str::FromStr, time::Instant};

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

        let coconuts = &numbers[1..];
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
    rounds: i32,
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

    pub fn play(&mut self) -> usize {
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

        self.monkeys
            .iter()
            .map(|m| m.borrow().odds + m.borrow().evens)
            .enumerate()
            .max_by(|curr, other| curr.1.cmp(&other.1))
            .unwrap()
            .0
    }
}

impl FromStr for CoconutGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: VecDeque<&str> = s.lines().collect();

        let rounds = lines
            .pop_front()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<i32>().ok())
            .flatten()
            .nth(0)
            .unwrap();

        let monkeys: Vec<_> = lines
            .iter()
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

pub fn game_from_file(input: &str) {
    let buffer = fs::read_to_string(input).unwrap();
    let stopwatch = Instant::now();
    let winner = game_from_buffer(&buffer);
    println!("{} winner: {} in {:?}", input, winner, stopwatch.elapsed());
}

fn game_from_buffer(buffer: &str) -> usize {
    let mut game: CoconutGame = buffer.parse().unwrap();
    return game.play();
}

pub fn game_from_folder(path: &str) {
    let dir = fs::read_dir(path).unwrap();

    let mut threads = Vec::new();

    let stopwatch = Instant::now();

    for file in dir.into_iter().map(|f| f.ok()).flatten() {
        let handler = std::thread::spawn(move || {
            game_from_file(&file.path().to_str().unwrap());
        });

        threads.push(handler);
    }

    for handler in threads {
        let _ = handler.join();
    }

    println!("\nElapsed: {:?}", stopwatch.elapsed());
}
