#![feature(proc_macro_hygiene, decl_macro)]
#![feature(crate_visibility_modifier)]
#![feature(label_break_value)]
#![warn(clippy::all)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate log;

mod authenticator;
mod login;
mod logout;

// Reexport so that everything is in the crate namespace
pub use self::authenticator::Authenticator;
pub use self::login::Login;
pub use self::logout::Logout;
