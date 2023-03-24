use monkeys_and_coconuts::{game_from_file, game_from_folder};
use std::env;

fn main() {
    let _ = env::args().nth(0).unwrap();

    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        game_from_folder("./casos");
    } else {
        let input_name = &args[0];
        game_from_file(input_name);
    }
}
