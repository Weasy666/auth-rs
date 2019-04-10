use super::Authenticator;
use rocket::data::{Data, FromData, Outcome, Transform, Transformed};
use rocket::http::uri::{FromUriParam, Query};
use rocket::http::{Cookie, Cookies, Status};
use rocket::outcome::Outcome::*;
use rocket::request::FormDataError;
use rocket::request::{FormItems, Request};
use rocket::response::{Flash, Redirect};
use std::ops::Deref;

/// This enum is used for authentication with a login form.
///
/// You just need to use it in a handler instead of Rockets [`Form`] type.
/// And use a type that implements [`Authenticator`] trait for its inner type.
/// 
/// # Example
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// use rocket_auth::{Authenticator, Login};
///
/// pub struct User {
///     pub username: String,
///     password: String,
/// }
/// impl Authenticator for User {
///     type Error = String;
///     type SessionKey = String;
///     type SessionToken = String;
/// 
///     fn session_key() -> Self::SessionKey {
///         "sid".into()
///     }
/// 
///     fn session_token(&self) -> Self::SessionToken {
///         crypto::generate_session_token()
///     }
/// 
///     fn authenticate(
///         &self,
///         request: &Request,
///         items: &mut FormItems,
///         _strict: bool,
///     ) -> Result<Login<Self>, Self::Error> {
///         // Get the values we need from the previously extracted FormItems
///         let (mut username, mut password, mut remember) = ("".into(), "".into(), false);
///         for form_item in items {
///             let (key, value) = form_item.key_value_decoded();
///             match key.as_str() {
///                 "username" | "email" => username = value,
///                 "password" => password = value,
///                 "remember" => remember = value == "on",
///                 _ => (),
///             }
///         }
/// 
///         // Check that we got some usable values
///         if username.is_empty() || password.is_empty() {
///             return Err("Invalid login form with missing field 'username' or 'password'.".into());
///         }
/// 
///         // Retrieve DB connection from request and authenticate user
///         let conn = request.guard::<DbConn>().unwrap();
///         let user = User::by_name_or_email(&conn, &username).map_err(|e| format!("User not found: {}", e))?;
///         let authenticated = crypto::password_verify(&user.salt, &user.password, &password);
/// 
///         if authenticated {
///             Ok(Login::Success(user))
///         } else {
///             Ok(Login::Failure(user))
///         }
///     }
/// }
/// 
/// #[post("/login", data = "<login>")]
/// fn login_post(login: Login<User>, cookies: Cookies) -> Redirect {
///     login.redirect("/", "/login", cookies)
/// }
/// 
/// fn main() { /* the regular Rocket init stuff... */ }
/// ```
/// 
/// [`Form`]: rocket::request::Form
/// [`Authenticator`]: authenticator::Authenticator
#[derive(Debug)]
pub enum Login<A> {
    Success(A),
    Failure(A),
}

impl<A: Authenticator> Deref for Login<A> {
    type Target = A;

    fn deref(&self) -> &A {
        match self {
            Login::Success(ref a) => a,
            Login::Failure(ref a) => a,
        }
    }
}

impl<'f, A: Authenticator> Login<A> {
    #[inline(always)]
    pub fn into_inner(self) -> A {
        match self {
            Login::Success(a) => a,
            Login::Failure(a) => a,
        }
    }

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

    /// Generates an appropriate response based on the login status that the authenticator returned.
    pub fn redirect<T: Into<String>, S: Into<String>>(
        &self,
        success_url: T,
        failure_url: S,
    ) -> Redirect {
        match self {
            Login::Success(_) => self.success(success_url),
            Login::Failure(_) => self.failure(failure_url),
        }
    }

    /// Generates an appropriate response based on the login status that the authenticator returned.
    /// In the case of an error an "error" `Flash` response is generated from the given `msg`.
    pub fn flash_redirect<T: Into<String>, S: Into<String>, R: Into<String>>(
        &self,
        success_url: T,
        failure_url: S,
        failure_msg: R,
    ) -> Result<Redirect, Flash<Redirect>> {
        match self {
            Login::Success(_) => Ok(self.success(success_url)),
            Login::Failure(_) => Err(self.flash_failure(failure_url, failure_msg)),
        }
    }

    crate fn from_login_form(
        request: &Request<'_>,
        form_str: &'f str,
        strict: bool,
    ) -> Outcome<Login<A>, FormDataError<'f, A::Error>> {
        use self::FormDataError::*;

        let mut items = FormItems::from(form_str);

        let result = A::authenticate(request, &mut items, strict);
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

    fn transform(request: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        use std::{cmp::min, io::Read};

        let outcome = 'o: {
            if !request.content_type().map_or(false, |ct| ct.is_form()) {
                warn_!("Form data does not have form content type.");
                break 'o Forward(data);
            }

            let limit = request.limits().get("forms").unwrap_or(4096);
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
