use std::{cell::RefCell, rc::Rc};

pub enum CoconutType {
    Even,
    Odd,
}

pub struct Monkey {
    even: usize,
    odd: usize,
    even_coconuts: Vec<usize>,
    odd_coconuts: Vec<usize>,
}

pub struct CoconoutGame {
    rounds: i32,
    monkeys: Vec<Rc<RefCell<Monkey>>>,
}

impl CoconoutGame {
    fn pass_coconuts(
        &self,
        monkey: Rc<RefCell<Monkey>>,
        other: Rc<RefCell<Monkey>>,
        coconut_type: CoconutType,
    ) {
        match coconut_type {
            CoconutType::Even => {
                while let Some(coconut) = monkey.borrow_mut().even_coconuts.pop() {
                    other.borrow_mut().even_coconuts.push(coconut);
                }
            }
            CoconutType::Odd => {
                while let Some(coconut) = monkey.borrow_mut().odd_coconuts.pop() {
                    other.borrow_mut().odd_coconuts.push(coconut);
                }
            }
        }
    }

    pub fn play(&mut self) {
        let mut rounds = self.rounds;

        while rounds > 0 {
            rounds -= 1;

            for i in 0..self.monkeys.len() {
                let monkey = self.monkeys[i].clone();

                let even_monkey = self.monkeys[monkey.borrow().even].clone();
                let odd_monkey = self.monkeys[monkey.borrow().odd].clone();

                self.pass_coconuts(monkey.clone(), even_monkey, CoconutType::Even);
                self.pass_coconuts(monkey, odd_monkey, CoconutType::Even);
            }
        }
    }
}
