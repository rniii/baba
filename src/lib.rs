//! Baba is an extremely simple engine for game development, inspired by love2d.
//!
//! Its main goal is to provide a robust base for games of any complexity. It is currently built
//! on top of [SDL2], which already has widespread usage and supports a huge variety of systems.
//!
//! Like SDL, it's entirely free. Baba uses the Apache license, which means usage for any purpose,
//! commercial or not, is allowed.
//!
//! [SDL2]: https://libsdl.org/
//!
//! ## Getting started
//!
//! All of the magic happens when you call [`baba::run`](run), and you can use the library right
//! away!
//!
//! ```no_run
//! # use baba::prelude::*;
//! fn main() -> baba::Result {
//!     baba::run("My game", MyGame::update)
//! }
//!
//! # #[derive(Default)]
//! # struct MyGame;
//! impl MyGame {
//!     fn update(&mut self) {
//!         // Update your game logic and draw onto the screen!
//!         gfx::clear(Color::WHITE);
//!     }
//! }
//! ```
//!
//! Refer to the [modules] to see what the engine can do. Baba is still pretty early in
//! development, so loads more documentation are still coming.
//!
//! [modules]: #modules
//!
//! ## Need help?

#![warn(
    clippy::pedantic,
    clippy::missing_const_for_fn,
    clippy::use_self,
    unsafe_op_in_unsafe_fn,
    missing_docs
)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::semicolon_if_nothing_returned,
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

mod error;
mod game;
pub mod gfx;
pub mod input;
pub mod math;
pub use error::{Error, SdlError};
pub use game::{Framerate, Game, Settings, WindowSettings};

/// A [`Result`][std::result] type for baba programs.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Simple entrypoint function for baba games.
///
/// If you don't currently need to set any engine options, this is simply a shorthand for that:
///
/// ```no_run
/// # #[derive(Default)]
/// # struct MyGame;
/// # impl MyGame { fn update(&mut self) {} }
/// # fn main() -> baba::Result {
/// baba::game("My game", MyGame::update)
///     .run()
/// # }
/// ```
///
/// See [`game`][game()] for more!
pub fn run<S>(name: impl Into<String>, update: impl Fn(&mut S)) -> Result
where
    S: Default,
{
    game(name, update).run()
}

/// Entrypoint function for baba games.
///
/// This function creates a [`Game`] object, which you can set many options in. To start your game,
/// simply call [`.run()`][Game::run] or [`.run_with(MyGame::new)`][Game::run_with] on it. Check
/// out [`Game`] for the options you can set!
///
/// ```no_run
/// fn main() -> baba::Result {
///     baba::game("My game", MyGame::update)
///         .window_title("Hello!")
///         .run()
/// }
///
/// #[derive(Default)]
/// struct MyGame {
///     // ...
/// }
/// # impl MyGame { fn update(&mut self) {} }
/// ```
///
/// If you don't want to implement [`Default`], you can instead use [`run_with`][Game::run_with]:
///
/// ```no_run
/// fn main() -> baba::Result {
///     baba::game("My game", MyGame::update)
///         .run_with(MyGame::new)
/// }
///
/// # struct MyGame;
/// # impl MyGame { fn update(&mut self) {} }
/// impl MyGame {
///     fn new() -> Self {
///         Self {
///             // ...
///         }
///     }
/// }
/// ```
pub fn game<S>(name: impl Into<String>, update: impl Fn(&mut S)) -> Game<S, impl Fn(&mut S)> {
    Game::new(name.into(), update)
}

/// Common functions and objects used by baba programs.
///
/// To avoid importing many things, it's recommended to glob import this on your files:
///
/// ```
/// use baba::prelude::*;
/// ```
pub mod prelude {
    #[doc(inline)]
    pub use crate::game::{Framerate, Settings, WindowSettings};
    #[doc(inline)]
    pub use crate::gfx::{
        self, Color, Drawable, Origin, ScaleMode, Texture, TextureOptions, TextureSlice, Transform,
        Vertex, Viewport, ViewportScaling,
    };
    #[doc(inline)]
    pub use crate::input::{self, is_key_down, is_key_pressed, KeyCode};
    #[doc(inline)]
    pub use crate::math::*;

    #[doc(inline)]
    pub use log::{debug, info, trace, warn};
}
