use std::fmt::Debug;
use crate::login::Login;
use crate::logout::Logout;
use rocket::request::FormItems;
use rocket::request::Request;

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
    type Error: Debug;

    fn get_cookie_key() -> String;

    fn authenticate(
        request: &Request,
        items: &mut FormItems,
        strict: bool,
    ) -> Result<Login<Self>, Self::Error>
    where
        Self: std::marker::Sized;

    fn logout(request: &Request) -> Result<Logout<Self>, Self::Error>
    where
        Self: std::marker::Sized;
}
