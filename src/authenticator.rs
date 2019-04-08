use crate::login::Login;
use rocket::request::FormItems;
use rocket::request::Request;
use std;

pub trait FromString {
    fn from_string(s: String) -> Self;
}

impl FromString for String {
    fn from_string(s: String) -> String {
        s
    }
}

/// This trait needs to be implemented by the type which will be used in [`Login`]
/// 
/// [`Login`]: crate::login::Login
pub trait Authenticator {
    type Error;
    type SessionKey: FromString + ToString;
    // Session requires an implementator of this type to also implement ToString
    // this because the type must be serializable into a string in order to store it inside a
    // cookie.
    type SessionToken: FromString + ToString;

    /// A function that returns the key with which the session token will be saved in the private cookie.
    fn session_key() -> Self::SessionKey;

    /// A function that returns a valid session token in the form of a String, which will
    /// be stored in a private cookie to identify a logged in user.
    fn session_token(&self) -> Self::SessionToken;

    fn authenticate(
        request: &Request,
        items: &mut FormItems,
        strict: bool,
    ) -> Result<Login<Self>, Self::Error>
    where
        Self: std::marker::Sized;
}
