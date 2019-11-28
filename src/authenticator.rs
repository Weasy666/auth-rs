use std::fmt::Debug;
use crate::login::Login;
use crate::logout::Logout;
use rocket::request::FormItems;
use rocket::request::Request;

/// This trait needs to be implemented by the type which will be used in [`Login`]
/// 
/// [`Login`]: crate::login::Login
pub trait Authenticator {
    type Error: Debug;

    /// Can be used as key to access this authenticators value in a cookie.
    /// The provided default implementation returns `sid` as key.
    fn get_cookie_key() -> String {
        "sid".to_string()
    }

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
