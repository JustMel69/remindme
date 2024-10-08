use crate::state::State;

pub fn run(msg: &str) {
    let mut state = State::load().expect("Couldn't load state.");
    if state.messages().len() == 0 {
        state.match_session();
    }

    if !state.matches_session() {
        eprintln!("error: Can't push message until all reminders have been popped.");
        std::process::exit(1);
    }

    state.push_msg(msg);
    state.save().expect("Couldn't save state.");
    println!("\x1b[1;37m remindme: Added reminder for next session.\x1b[0m")
}
