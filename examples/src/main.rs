#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_auth as auth;

mod dummy;

use rocket::response::Flash;
use auth::{ AuthUser, Login };
use rocket::request::{ FlashMessage };
use rocket::response::Redirect;
use rocket::response::content::Html;
use rocket::http::Cookies;
use dummy::DummyUser;

#[get("/admin")]
fn admin(info: DummyUser) -> Html<String> {
	// we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
	Html(format!("Restricted administration area, user logged in: {}, <a href=\"/logout\" >Logout</a> ", info.username))
}

#[get("/admin", rank = 2)]
fn login(msg: Option<FlashMessage>) -> Html<String> {
    let message = match msg {
        Some(ref msg) => msg.msg(),
        None => "",
    };
     
    Html(format!(
    "<form action=\"/admin\" method=\"POST\">
        Regular Login
        <input type=\"text\" name=\"username\" />
        <input type=\"password\" name=\"password\" />
        <input type=\"submit\" value=\"Login\" />
    </form>
    
    <form action=\"/admin_flash\" method=\"POST\">
        Flash Message Login
        <input type=\"text\" name=\"username\" />
        <input type=\"password\" name=\"password\" />
        <input type=\"submit\" value=\"Login\" />
        <small>
               {}
        </small>
    </form>", message))
}

#[get("/logout",)]
fn logout(info: DummyUser, mut cookies: Cookies) -> Redirect {
    info.logout(&mut cookies);
    Redirect::to("/admin")
}

#[post("/admin", data = "<login>")]
fn login_post(login: Login<DummyUser>, cookies: Cookies) -> Redirect {
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login.
	login.redirect("/admin", "/admin", cookies)
}

#[post("/admin_flash", data = "<login>")]
fn login_post_flash(login: Login<DummyUser>, cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login and the message is what gets send as
    // a Flash message cookie to the client.
	login.flash_redirect("/admin", "/admin", "Wrong password", cookies)
}

fn main() {
    // main setup code
    rocket::ignite()
        .mount("/", routes![admin, login, login_post, login_post_flash, logout])
        .launch();
}