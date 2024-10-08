use crate::state::State;

pub fn run() {
    let mut state = State::load().expect("Couldn't load state.");
    if state.matches_session() {
        eprintln!("error: Expired session, can't display reminders.");
        std::process::exit(1);
    }

    if state.messages().len() == 0 {
        println!("No reminders left.")
    }

    for (i, msg) in state.messages().iter().enumerate() {
        println!("\x1b[1;36m[{i}]:\x1b[0m {msg}");
    }

    state.clear_messages();
    state.save().expect("Couldn't save state.");
}
