use std::{env, fs, thread, time::Instant};

use monkeys_and_coconuts::CoconutGame;

fn game_from_especific_file(input: &str) {
    let buffer = fs::read_to_string(input).unwrap();

    let mut game: CoconutGame = buffer.parse().unwrap();
    let (winner, coconuts) = game.play();

    println!(
        "{} winner: {} with {} cocunuts in {} rounds!",
        input, winner, coconuts, game.rounds
    );
}

fn game_from_folder(folder: &str) {
    let dir = fs::read_dir(folder).unwrap();

    let mut threads = Vec::new();

    let stopwatch = Instant::now();

    for file in dir.into_iter().map(|f| f.ok()).flatten() {
        let handler = thread::spawn(move || {
            game_from_especific_file(&file.path().to_str().unwrap());
        });

        threads.push(handler);
    }

    for handler in threads {
        let _ = handler.join();
    }

    println!("Elapsed: {:?}", stopwatch.elapsed());
}

fn main() {
    let _ = env::args().nth(0).unwrap();

    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        game_from_folder("./casos");
    } else {
        let input_name = &args[0];
        game_from_especific_file(input_name);
    }
}
