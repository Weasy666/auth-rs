#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_simpleauth as auth;
#[macro_use] extern crate rocket;

mod dummy;

use auth::userpass::UserPass;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::response::content::Html;
use rocket::http::Cookies;
use auth::status::{LoginStatus,LoginRedirect};
use dummy::DummyAuthenticator;

#[get("/admin")]
fn admin(info: UserPass<String>) -> Html<String> {
	// we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
	Html(format!("Restricted administration area, user logged in: {}, <a href=\"/logout\" >Logout</a> ", info.user))
}


#[get("/admin", rank = 2)]
fn login() -> Html<&'static str>{
    Html(
    "<form action=\"/admin\" method=\"POST\"> 
        <input type=\"text\" name=\"username\" />
        <input type=\"password\" name=\"password\" />
        <input type=\"submit\" value=\"Login\" />
    </form>"
    )
}

#[get("/logout",)]
fn logout(mut info: UserPass<String>) -> Redirect {
    info.logout();
    Redirect::to("/admin")
}

#[post("/admin", data = "<form>")]
fn login_post(form: Form<LoginStatus<DummyAuthenticator>>, cookies: Cookies) -> LoginRedirect{
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login
	form.into_inner().redirect("/admin", "/admin", cookies)
}

fn main(){
    // main setup code
    rocket::ignite().mount("/", routes![admin,login, login_post, logout]).launch();
}
