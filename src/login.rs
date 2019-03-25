use std::ops::Deref;
use rocket::response::{ Flash, Redirect, };
use rocket::request::{ FormItems, Request };
use rocket::http::{ Status, Cookie, Cookies };
use super::Authenticator;
use rocket::outcome::Outcome::*;
use rocket::request::{FormDataError};
use rocket::data::{Outcome, Transform, Transformed, Data, FromData};
use rocket::http::{uri::{Query, FromUriParam}};

pub enum Login<A> {
    Success(A),
    Failure(A)
}

impl<A: Authenticator> Deref for Login<A> {
    type Target = A;

    fn deref(&self) -> &A {
        //&self.0
        match self {
          Login::Success(ref a) => a,
          Login::Failure(ref a) => a
        }
    }
}

impl<'f, A: Authenticator> Login<A> {
    #[inline(always)]
    pub fn into_inner(self) -> A {
        match self {
          Login::Success(a) => a,
          Login::Failure(a) => a
        }
    }

    /// Generates a succeed response
    fn success<T: Into<String>>(self, url: T, mut cookies: Cookies) -> Redirect {
        //TODO: use RocketConfig when this gets integrated into Rocket
        let cookie_id = super::authenticator::cookie_auth_key();

        cookies.add_private(Cookie::new(cookie_id, self.deref().session_id().to_string()));
        Redirect::to(url.into())
    }

    /// Generates a failed response
    fn failure<T: Into<String>>(self, url: T) -> Redirect {
        Redirect::to(url.into())
    }

    /// Generates a failed response
    fn flash_failure<T: Into<String>, S: Into<String>>(self, url: T, msg: S) -> Flash<Redirect> {
        Flash::error(Redirect::to(url.into()), msg.into())
    }

    /// Generate an appropriate response based on the login status that the authenticator returned
    pub fn redirect<T: Into<String>, S: Into<String>>(self, success_url: T, failure_url: S, cookies: Cookies) -> Redirect {
        match self {
          Login::Success(_) => self.success(success_url, cookies),
          Login::Failure(_) => self.failure(failure_url)
        }
    }

    /// Generate an appropriate response based on the login status that the authenticator returned
    pub fn flash_redirect<T: Into<String>, S: Into<String>, R: Into<String>>(self, success_url: T, failure_url: S, failure_msg: R, cookies: Cookies) -> Result<Redirect,Flash<Redirect>> {
        match self {
          Login::Success(_) => Ok(self.success(success_url, cookies)),
          Login::Failure(_) => Err(self.flash_failure(failure_url, failure_msg))
        }
    }

    crate fn from_login_form(request: &Request<'_>, form_str: &'f str, strict: bool) -> Outcome<Login<A>, FormDataError<'f, A::Error>> {
        use self::FormDataError::*;

        let mut items = FormItems::from(form_str);

        let result = A::try_login(request, &mut items, strict);
        if !items.exhaust() {
            error_!("The request's form string was malformed.");
            return Failure((Status::BadRequest, Malformed(form_str)));
        }

        match result {
            Ok(v) => Success(v),
            Err(e) => {
                error_!("Incorrect user or password was given.");
                Failure((Status::Unauthorized, Parse(e, form_str)))
            }
        }
    }
}

impl<'f, A: Authenticator> FromData<'f> for Login<A> {
    type Error = FormDataError<'f, A::Error>;
    type Owned = String;
    type Borrowed = str;

    fn transform(
        request: &Request,
        data: Data
    ) -> Transform<Outcome<Self::Owned, Self::Error>> {
        use std::{cmp::min, io::Read};

        let outcome = 'o: {
            if !request.content_type().map_or(false, |ct| ct.is_form()) {
                warn_!("Form data does not have form content type.");
                break 'o Forward(data);
            }
            //TODO: uncomment when this gets integrated into Rocket
            let limit = 4096;//request.limits().forms;
            let mut stream = data.open().take(limit);
            let mut form_string = String::with_capacity(min(4096, limit) as usize);
            if let Err(e) = stream.read_to_string(&mut form_string) {
                break 'o Failure((Status::InternalServerError, FormDataError::Io(e)));
            }

            break 'o Success(form_string);
        };

        Transform::Borrowed(outcome)
    }

    fn from_data(req: &Request, o: Transformed<'f, Self>) -> Outcome<Self, Self::Error> {
        <Login<A>>::from_login_form(req, o.borrowed()?, true)
    }
}

impl<'f, A, T: FromUriParam<Query, A>> FromUriParam<Query, A> for Login<T> {
    type Target = T::Target;
 
    #[inline(always)]
    fn from_uri_param(param: A) -> Self::Target {
        T::from_uri_param(param)
    }
}