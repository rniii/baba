//! Input handling.
//!
//! Currently provides keyboard support with [`is_key_pressed`], [`is_key_down`],
//! [`get_pressed_keys`] and [`get_held_keys`]

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

/// Was this key pressed this frame?
#[must_use]
pub fn is_key_pressed(key: KeyCode) -> bool {
    INPUT_STATE.lock().just_pressed.contains(&key)
}

/// Is this key being held down?
#[must_use]
pub fn is_key_down(key: KeyCode) -> bool {
    INPUT_STATE.lock().pressed.contains(&key)
}

/// Get a list of keys pressed within this frame.
pub fn get_pressed_keys() -> impl ExactSizeIterator<Item = KeyCode> {
    INPUT_STATE.lock().just_pressed.clone().into_iter()
}

/// Get a list of keys currently being held down.
pub fn get_held_keys() -> impl ExactSizeIterator<Item = KeyCode> {
    INPUT_STATE.lock().pressed.clone().into_iter()
}

/// Simulate pressing a key.
///
/// [`is_key_pressed`] will return `true` for this frame, and [`is_key_down`] will return `true` until you call [`release_key`].
pub fn press_key(key: KeyCode) {
    let mut input = INPUT_STATE.lock();
    input.pressed.insert(key);
    input.just_pressed.insert(key);
}

/// Simulate releasing a key.
///
/// [`is_key_down`] will stop returning `true` for this key.
pub fn release_key(key: KeyCode) {
    INPUT_STATE.lock().pressed.remove(&key);
}

/// Clears all keys pressed this frame.
///
/// Data for [`is_key_pressed`] will be cleared.
pub fn clear() {
    INPUT_STATE.lock().just_pressed.clear();
}
