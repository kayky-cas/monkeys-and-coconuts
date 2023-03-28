use monkeys_and_coconuts::io::run;
use std::env;

fn main() {
    let _ = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    run(&args);
}
