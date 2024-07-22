use std::cell::RefCell;
use std::collections::HashSet;


thread_local! {
    static INPUT_STATE: RefCell<InputState> = RefCell::default();
}
mod keycode;
pub use keycode::KeyCode;

#[derive(Default)]
struct InputState {
    pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
}

#[must_use]
pub fn is_key_pressed(key: KeyCode) -> bool {
    INPUT_STATE.with_borrow(|input| input.just_pressed.contains(&key))
}

#[must_use]
pub fn is_key_down(key: KeyCode) -> bool {
    INPUT_STATE.with_borrow(|input| input.pressed.contains(&key))
}

pub fn press_key(key: KeyCode) {
    INPUT_STATE.with_borrow_mut(|input| {
        input.pressed.insert(key);
        input.just_pressed.insert(key);
    });
}

pub fn release_key(key: KeyCode) {
    INPUT_STATE.with_borrow_mut(|input| {
        input.pressed.remove(&key);
    })
}

pub fn clear() {
    INPUT_STATE.with_borrow_mut(|input| {
        input.just_pressed.clear();
    })
}
