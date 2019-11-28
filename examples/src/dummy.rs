use auth::Authenticator;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::Outcome;
use rocket::request::FormItems;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket_auth::{Login, Logout};

pub struct DummySession {
    pub token: String,
}

impl DummySession {
    fn new() -> DummySession {
        DummySession::default()
    }
}

impl Default for DummySession {
    fn default() -> Self{
        DummySession {
            token: "12a34b56c".into()
        }
    }
}


pub struct DummyUser {
    pub username: String,
}

/// An implementation of the authenticator which always lets the authentication succeed
///
/// On every invocation this will also print the incoming username and password.
///
/// This type should only be used for testing purposes.
impl Authenticator for DummyUser {
    type Error = String;

    fn authenticate(
        request: &Request,
        items: &mut FormItems,
        _strict: bool,
    ) -> Result<Login<Self>, Self::Error> {
        // Get the values we need form the previously extracted FormItems
        let (mut username, mut password, mut remember) = ("".into(), "".into(), false);
        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "username" | "email" => username = value,
                "password" => password = value,
                "remember" => remember = value == "on",
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

        if remember {
            if let Some(mut cookies) = request.guard::<Cookies>().succeeded() {
                let session = DummySession::new();
                // Add session into DB
                cookies.add_private(Cookie::new(
                    Self::get_cookie_key(),
                    session.token,
                ));
            }
        }

        if authenticated {
            Ok(Login::Success(user))
        } else {
            Ok(Login::Failure(user))
        }
    }

    fn logout(request: &Request) -> Result<Logout<Self>, Self::Error> {
        // Get the session cookie from the current request
        let mut user = None;
        if let Some(mut cookies) = request.guard::<Cookies>().succeeded() {
            if let Some(_sid) = cookies.get_private(&Self::get_cookie_key()) {
                // Retrieve session and associated user from Db
                let db_retrieved_username = "Isaac".into();
                user = Some(DummyUser { username: db_retrieved_username });
                // when everything went well, we also need to delete the session from the cookie
                cookies.remove_private(Cookie::named(Self::get_cookie_key()));
            }
        }

        match user {
            Some(user) => Ok(Logout::Success(user)),
            None => Ok(Logout::Failure)
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
        match request.cookies().get_private(&Self::get_cookie_key()) {
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
