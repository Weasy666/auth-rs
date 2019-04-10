use std::convert::TryInto;
use super::Authenticator;
use rocket::http::{Status, uri::Uri};
use rocket::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{Flash, Redirect};

/// This enum is used to logout an authenticated/logged in user.
///
/// You just need to wrap your user struct with this type in a handler and implement
/// the [`Authenticator`] trait for it.
/// 
/// # Example
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// use rocket_auth::{Authenticator, Logout};
///
/// pub struct User {
///     pub username: String,
///     password: String,
/// }
/// impl Authenticator for User {
///     type Error = String;
/// 
///     fn get_cookie_key() -> String {
///         "sid".into()
///     }
/// 
///     fn logout(request: &Request) -> Result<Logout<Self>, Self::Error> {
///         let mut cookies = request.cookies();
///         let sid = cookies.get_private(&Self::get_cookie_key())
///             .ok_or_else(|| "No user or session for the provided cookie found!".to_string())?;
/// 
///         cookies.remove_private(Cookie::named(Self::get_cookie_key()));
/// 
///         // Retrieve DB connection from request and delete session from DB
///         // and load the User whose session you deleted, so that we can return him
///         let user = User { username: "Isaac".into(), password: "".into() };
///         
///         match user {
///             Ok(user) => Ok(Logout::Success(user)),
///             _        => Ok(Logout::Failure),
///         }
///     }
/// }
/// 
/// #[get("/logout")]
/// fn logout(logout: Logout<User>) -> Redirect {
///     logout.redirect("/", "/")
/// }
/// 
/// fn main() { /* the regular Rocket init stuff... */ }
/// ```
/// 
/// [`Form`]: rocket::request::Form
/// [`Authenticator`]: authenticator::Authenticator
#[derive(Debug)]
pub enum Logout<A> {
    Success(A),
    Failure,
}

impl<'f, A: Authenticator> Logout<A> {

    /// Generates a success response.
    fn success<T: TryInto<Uri<'static>>>(&self, url: T) -> Redirect {
        Redirect::to(url)
    }

    /// Generates a failure response.
    fn failure<T: TryInto<Uri<'static>>>(&self, url: T)  -> Redirect {
        Redirect::to(url)
    }

    /// Generates a failure response with a "error" `Flash` message from the given `msg`.
    fn flash_failure<T: TryInto<Uri<'static>>, S: Into<String>>(&self, url: T, msg: S) -> Flash<Redirect> {
        Flash::error(Redirect::to(url), msg.into())
    }

    /// Generates an appropriate response based on the Logout status that the authenticator returned.
    pub fn redirect<T: TryInto<Uri<'static>>, S: TryInto<Uri<'static>>>(
        &self,
        success_url: T,
        failure_url: S,
    ) -> Redirect {
        match self {
            Logout::Success(_) => self.success(success_url),
            Logout::Failure => self.failure(failure_url),
        }
    }

    /// Generates an appropriate response based on the Logout status that the authenticator returned.
    /// In the case of an error an "error" `Flash` response is generated from the given `msg`.
    pub fn flash_redirect<T: TryInto<Uri<'static>>, S: TryInto<Uri<'static>>, R: Into<String>>(
        &self,
        success_url: T,
        failure_url: S,
        failure_msg: R,
    ) -> Result<Redirect, Flash<Redirect>> {
        match self {
            Logout::Success(_) => Ok(self.success(success_url)),
            Logout::Failure => Err(self.flash_failure(failure_url, failure_msg)),
        }
    }

    crate fn from_logout(request: &Request<'_>) -> Outcome<Logout<A>, LogoutError> {
        let result = A::logout(request);

        match result {
            Ok(v) => Success(v),
            Err(e) => {
                error_!("Something went wrong when logging out.");
                Failure((Status::InternalServerError, LogoutError(format!("Something went wrong when logging out: {:?}", e))))
            }
        }
    }
}

#[derive(Debug)]
pub struct LogoutError(String);

impl<'a, 'r, A: Authenticator> FromRequest<'a, 'r> for Logout<A> {
    type Error = LogoutError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        <Logout<A>>::from_logout(request)
    }
}