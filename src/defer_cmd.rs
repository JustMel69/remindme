use crate::state::State;

pub fn run() {
    let mut state = State::load().expect("Couldn't load state.");
    if state.matches_session() {
        eprintln!("error: Expired session, can't defer reminders.");
        std::process::exit(1);
    }

    if state.messages().len() == 0 {
        println!("No reminders to defer.")
    }

    state.match_session();
    println!("\x1b[1;37mReminders defered.\x1b[0m");

    state.save().expect("Couldn't save state.");
}
