use auth::Authenticator;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::Outcome;
use rocket::request::FormItems;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket_auth::Login;

pub struct DummyUser {
    pub username: String,
}

impl DummyUser {
    pub fn logout(&self, cookies: &mut Cookies) {
        // Normally her would be some code to also log the user out in the DB
        cookies.remove_private(Cookie::named(Self::session_key()));
    }
}

/// An implementation of the authenticator which always lets the authentication succeed
///
/// On every invocation this will also print the incoming username and password.
///
/// This type should only be used for testing purposes.
impl Authenticator for DummyUser {
    type Error = String;
    type SessionKey = String;
    type SessionToken = String;

    fn session_key() -> Self::SessionKey {
        "sid".into()
    }

    fn session_token(&self) -> Self::SessionToken {
        "12a34b56c".into()
    }

    fn authenticate(
        _request: &Request,
        items: &mut FormItems,
        _strict: bool,
    ) -> Result<Login<Self>, Self::Error> {
        // Get the values we need form the previously extracted FormItems
        let (mut username, mut password) = ("".into(), "".into());
        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "username" => username = value,
                "password" => password = value,
                _ => (),
            }
        }

        // Check that we got some usable values
        if username.is_empty() || password.is_empty() {
            return Err("Invalid login form with missing fiel 'username' or 'password'.".into());
        }

        let user = DummyUser { username };
        println!("Authenticating user: {}", user.username);

        // Retrieve DB connection from request and do some authentication
        let authenticated = true;

        if authenticated {
            Ok(Login::Success(user))
        } else {
            Ok(Login::Failure(user))
        }
    }
}

/// A request guard that checks if a private cookie was provided   
///
/// The name of the cookie can be configured with simpleauth_cookie_identifier config key in your
/// Rocket config file.
///
/// By default it is "sid" see the config module
impl<'a, 'r> FromRequest<'a, 'r> for DummyUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<DummyUser, Self::Error> {
        match request.cookies().get_private(&Self::session_key()) {
            Some(_sid) => {
                // Retrieve DB connection from request, check if sessionID is valid and get user data from DB
                let db_retrieved_username = "Isaac".into();
                Outcome::Success(DummyUser {
                    username: db_retrieved_username,
                })
            }
            None => Outcome::Forward(()),
        }
    }
}
