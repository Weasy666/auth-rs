#![warn(clippy::all)]
extern crate rocket;

mod authenticator;
mod authuser;
mod login;

// Reexport so that everything is in the crate namespace
pub use self::authuser::{ AuthUser, FromString };
pub use self::authenticator::{ Authenticator };
pub use self::login::{ Login, LoginRedirect };