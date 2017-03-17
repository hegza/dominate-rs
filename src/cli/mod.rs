mod text;
mod actions;

use std::io::{self, BufRead, Write};

use self::text::*;
use self::actions::*;

pub struct CLI {
    file: String,
}

impl CLI {
    pub fn new(file: &str) -> CLI {
        CLI { file: String::from(file) }
    }

    pub fn run(&mut self) {

        loop {
            // Read preferred action
            let input = read_buffer("> ").unwrap();
            let parsed_action = parse_keyword(&input);

            if parsed_action.is_none() {
                println!("unknown pattern: {},\navailable commands: display, help, quit",
                         input);
                continue;
            }

            // Act
            if !self.act(parsed_action.unwrap()) {
                break;
            }

            // Output a newline at the end of the loop, for clarity
            println!("");
        }

    }

    fn act(&mut self, action: Keyword) -> bool {
        match action {
            Keyword::Display => println!("{}\nEOF", self.file),
            Keyword::Help => println!("{}", HELP),
            Keyword::Quit => {
                println!("bye!");
                return false;
            }
        }

        true
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
