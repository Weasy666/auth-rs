use super::Authenticator;
use rocket::http::{Status};
use rocket::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{Flash, Redirect};

pub enum Logout<A> {
    Success(A),
    Failure,
}

impl<'f, A: Authenticator> Logout<A> {

    /// Generates a success response.
    fn success<T: Into<String>>(&self, url: T) -> Redirect {
        Redirect::to(url.into())
    }

    /// Generates a failure response.
    fn failure<T: Into<String>>(&self, url: T) -> Redirect {
        Redirect::to(url.into())
    }

    /// Generates a failure response with a "error" `Flash` message from the given `msg`.
    fn flash_failure<T: Into<String>, S: Into<String>>(&self, url: T, msg: S) -> Flash<Redirect> {
        Flash::error(Redirect::to(url.into()), msg.into())
    }

    /// Generates an appropriate response based on the Logout status that the authenticator returned.
    pub fn redirect<T: Into<String>, S: Into<String>>(
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
    pub fn flash_redirect<T: Into<String>, S: Into<String>, R: Into<String>>(
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

    crate fn from_logout(
        request: &Request<'_>,
    ) -> Outcome<Logout<A>, LogoutError> {

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