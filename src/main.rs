use std::env;

const USAGE: &'static str = r#"Usage: remindme [OPTIONS] [MSG]

    --warn  Displays a warning if there are reminders on the stack
    --show  Displays and clears the reminders
    --init  Initializes a new session
    --help  Displays this message
"#;

mod warn_cmd;
mod show_cmd;
mod push_cmd;
mod init_cmd;
mod state;

fn main() {
    let mut args = env::args();
    args.next();

    match args.next().as_ref().map(|x| x.trim()) {
        Some("--warn") => warn_cmd::run(),
        Some("--show") => show_cmd::run(),
        Some("--init") => init_cmd::run(),
        Some("--help") => help_cmd(),
        Some(x) => {
            if x.starts_with("--") {
                eprintln!("error: Unknown command \"{x}\"! Use remindme --help for usage.");
                std::process::exit(1);
            }

            push_cmd::run(x);
        },
        None => {
            eprintln!("error: Expected a message or a command! Use remindme --help for usage.");
            std::process::exit(1);
        }
    }
}

fn help_cmd() {
    println!("{}", USAGE);
}
