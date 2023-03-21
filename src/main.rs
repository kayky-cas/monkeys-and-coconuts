use std::{env, fs, thread};

use anyhow::Error;
use monkeys_and_coconuts::CoconutGame;

fn game_from_especific_file(input: &str) -> Result<(), Error> {
    let buffer = fs::read_to_string(input)?;

    let mut game: CoconutGame = buffer.parse()?;

    // let winner = match game.play() {
    //     Some(w) => w,
    //     None => todo!(),
    // };
    //
    let winner = game.play();

    println!("{} winner: {}", input, winner);

    Ok(())
}

fn game_from_folder(folder: &str) -> Result<(), Error> {
    let dir = fs::read_dir(folder)?;

    let mut threads = Vec::new();

    for file in dir.into_iter().map(|f| f.ok()).flatten() {
        let tr = thread::spawn(move || {
            let _ = game_from_especific_file(&file.path().to_str().unwrap());
        });

        threads.push(tr);
    }

    loop {
        if threads.iter().filter(|tr| !tr.is_finished()).count() == 0 {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let _program_name = env::args()
        .nth(0)
        .ok_or(anyhow::anyhow!("Invalid program name"))?;

    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        game_from_folder("./casos")?;

        return Ok(());
        //return Err(anyhow::anyhow!(
        //    "{} needs at least 1 argument to execute.",
        //    program_name
        //));
    }

    let input_name = &args[0];

    game_from_especific_file(input_name)?;

    Ok(())
}
