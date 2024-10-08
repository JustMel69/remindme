use crate::state::State;

pub fn run() {
    let mut state = State::load().expect("Couldn't load state.");
    state.next_session();

    state.save().expect("Couldn't save state.");
}
