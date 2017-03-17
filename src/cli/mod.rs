mod tokens;
mod text;
mod actions;

use std::io::{self, BufRead, Write};

use self::text::*;
use self::tokens::*;
use self::actions::*;

pub fn run() {

    loop {
        // Read preferred action
        let input = read_buffer("> ").unwrap();
        let action = resolve_action(&input);

        // Act
        match action {
            TopLevelAction::Help => println!("{}", HELP),
            TopLevelAction::Quit => {
                println!("bye!");
                break;
            }
            TopLevelAction::Unknown => {
                println!("unknown pattern: {},\navailable commands: help, quit",
                         input);
            }
        }

        // Output a newline at the end of the loop, for clarity
        println!("");
    }
}

fn resolve_action(input: &str) -> TopLevelAction {
    match input.as_ref() {
        HELP_TOKEN => TopLevelAction::Help,
        EXIT_TOKEN => TopLevelAction::Quit,
        _ => TopLevelAction::Unknown,
    }
}

fn read_buffer(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    // Print the prompt
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;

    // Drop newline '\n'
    buffer.pop();
    Ok(buffer)
}
