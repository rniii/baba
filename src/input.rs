use std::collections::BTreeSet;

use parking_lot::Mutex;

mod keycode;
pub use keycode::KeyCode;

struct InputState {
    pressed: BTreeSet<KeyCode>,
    just_pressed: BTreeSet<KeyCode>,
}

static INPUT_STATE: Mutex<InputState> = Mutex::new(InputState {
    pressed: BTreeSet::new(),
    just_pressed: BTreeSet::new(),
});

#[must_use]
pub fn is_key_pressed(key: KeyCode) -> bool {
    INPUT_STATE.lock().just_pressed.contains(&key)
}

#[must_use]
pub fn is_key_down(key: KeyCode) -> bool {
    INPUT_STATE.lock().pressed.contains(&key)
}

pub fn press_key(key: KeyCode) {
    let mut input = INPUT_STATE.lock();
    input.pressed.insert(key);
    input.just_pressed.insert(key);
}

pub fn release_key(key: KeyCode) {
    INPUT_STATE.lock().pressed.remove(&key);
}

pub fn clear() {
    INPUT_STATE.lock().just_pressed.clear();
}
