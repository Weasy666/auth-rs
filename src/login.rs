use rocket::Response;
use rocket::response::{ Flash, Redirect, Responder };
use rocket::request::{ FormItems, FromForm, Request };
use rocket::http::{ Status, Cookie, Cookies };
use std::collections::HashMap;
use super::Authenticator;


/// Login state is used after the user has typed its username and password. It checks with an
/// authenticator if given credentials are valid and returns InvalidCredentials or Succeed based
/// on the validality of the username and password.
///
/// It does that by implementing the FromForm trait that takes the form submitted by your login
/// page
///
/// It expects a form like this on your page:
///
///```
///<form>
/// <input type="text" name="username" />
/// <input type="password" name="password" />
///</form>
/// ```
pub enum Login<A> {
    Success(A),
    Failure(A)
}

impl<A: Authenticator> Login<A> {
    /// Returns the user id from an instance of Authenticator
    pub fn get_authenticator (&self) -> &A {
        match self {
            Login::Success(ref authenticator) => authenticator,
            Login::Failure(ref authenticator) => authenticator
        }
    }

    /// Generates a succeed response
    fn success<T: Into<String>>(self, url: T, mut cookies: Cookies) -> Redirect {
        let cookie_id = super::authenticator::cookie_id();

        cookies.add_private(Cookie::new(cookie_id, self.get_authenticator().user().to_string()));
        Redirect::to(url.into().to_string())
    }

    /// Generates a failed response
    fn failure<T: Into<String>>(self, url: T) -> Redirect {
        Redirect::to(url.into().to_string())
    }

    /// Generates a failed response
    fn flash_failure<T: Into<String>, S: Into<String>>(self, url: T, msg: S) -> Flash<Redirect> {
        Flash::error(Redirect::to(url.into().to_string()), msg.into())
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
}

impl<'f,A: Authenticator> FromForm<'f> for Login<A> {
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        let mut user_pass = HashMap::new();

        for form_item in form_items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "username" => user_pass.insert("username", value).map_or((), |_v| ()),
                "password" => user_pass.insert("password", value).map_or((), |_v| ()),
                _ => ()
            }
        }

        if user_pass.get("username").is_none() || user_pass.get("password").is_none() {
            Err("invalid form")
        } else {
            let result = A::try_login(user_pass["username"].to_string(), user_pass["password"].to_string());

            Ok(match result {
                Ok(authenticator) => Login::Success(authenticator),
                Err(authenticator) => Login::Failure(authenticator)
            })
        }
    }
}