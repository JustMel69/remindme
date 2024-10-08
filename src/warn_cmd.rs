use crate::state::State;

pub fn run() {
    let state = State::load().expect("Couldn't load state.");
    if !state.matches_session() && state.messages().len() > 0 {
        println!("\x1b[1;33mremindme: You have {} unread reminders.\x1b[0m", state.messages().len());
    }
}
