use std::{fs, thread, time::Instant};

use crate::coconuts::CoconutGame;

fn game_from_especific_file(input: &str) {
    let buffer = fs::read_to_string(input).unwrap();

    let stopwatch = Instant::now();

    let mut game: CoconutGame = buffer.parse().unwrap();
    let (winner, coconuts) = game.play();

    println!(
        "{} winner: {} with {} coconuts in {} rounds! {:?}",
        input,
        winner,
        coconuts,
        game.rounds,
        stopwatch.elapsed()
    );
}

fn game_from_folder(folder: &str) {
    let dir = fs::read_dir(folder).unwrap();

    let mut files: Vec<_> = dir
        .into_iter()
        .map(|f| f.ok())
        .flatten()
        .map(|f| f.path().to_str().unwrap().to_owned())
        .collect();

    files.sort();

    let stopwatch = Instant::now();

    for path in files {
        game_from_especific_file(&path);
    }

    println!("Elapsed: {:?}", stopwatch.elapsed());
}

fn game_from_folder_multi(folder: &str) {
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

pub fn run(args: &[String]) {
    if args.is_empty() {
        game_from_folder("./casos");
    } else {
        let input_name = &args[0];

        match input_name.as_str() {
            "-t" => game_from_folder_multi("./casos"),
            _ => game_from_especific_file(input_name),
        }
    }
}
