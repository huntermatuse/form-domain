// i refer to keep `main.rs` and `lib.rs` separate
// as it makes it easier to add extra helper
// binaries later which share code with the main project.

/// Defines the arguments required to start the server application using [`clap`].
///
/// [`clap`]: https://github.com/clap-rs/clap/
pub mod config;

pub mod http;
