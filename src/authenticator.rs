use crate::login::Login;
use rocket::request::Request;
use rocket::request::FormItems;
use std;

pub trait FromString {
    fn from_string(s: String) -> Self;
}

impl FromString for String {
    fn from_string(s: String) -> String{
        s
    }
}

pub trait Authenticator{
    type Error;
    // The type that is returned when someone calls user() on the authenticator
    // this can for example be a structure that represent the user that is logged in.
    // 
    // LoginStatus requires an implementator of this type to also implement ToString
    // this because the type must be serializable into a string in order to store it inside a
    // cookie.
    //
    // UserPass requires an implementator of this type to also implement FromString in order to
    // retreive the type back from the cookie string
    type Session: FromString + ToString;

    /// a function that returns a UserToken in the form of a String, which identifies a user from a cookie.
    fn session_id(&self) -> Self::Session;

    fn try_login(request: &Request, items: &mut FormItems, strict: bool) -> Result<Login<Self>, Self::Error>
        where Self: std::marker::Sized;
}

type CookieID = String;

/// Returns the key for the cookie used to authenticate a user.
//TODO: find better solution -> maybe config after it was refactored in Rocket, see Rocket issue #852.
pub fn cookie_auth_key() -> CookieID {
    "sid".into()
}