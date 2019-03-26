// use std::ops::Deref;
// use rocket::Response;
// use rocket::response::{ Flash, Redirect, Responder };
// use rocket::request::{ FormItems, FromForm, Request };
// use rocket::http::{ Status, Cookie, Cookies };
// use std::collections::HashMap;
// use super::Authenticator;
// use rocket::outcome::Outcome::*;
// use rocket::request::{FormDataError};
// use rocket::data::{Outcome, Transform, Transformed, Data, FromData};
// use rocket::http::{uri::{Query, FromUriParam}};

// /// Login state is used after the user has typed its username and password. It checks with an
// /// authenticator if given credentials are valid and returns InvalidCredentials or Succeed based
// /// on the validality of the username and password.
// ///
// /// It does that by implementing the FromForm trait that takes the form submitted by your login
// /// page
// ///
// /// It expects a form like this on your page:
// ///
// ///```
// ///<form>
// /// <input type="text" name="username" />
// /// <input type="password" name="password" />
// ///</form>
// /// ```
// // pub enum Login<A> {
// //     Success(A),
// //     Failure(A)
// // }

// // impl<A: Authenticator> Login<A> {
// //     /// Returns the user id from an instance of Authenticator
// //     pub fn get_authenticator (&self) -> &A {
// //         match self {
// //             Login::Success(ref authenticator) => authenticator,
// //             Login::Failure(ref authenticator) => authenticator
// //         }
// //     }

// //     /// Generates a succeed response
// //     fn success<T: Into<String>>(self, url: T, mut cookies: Cookies) -> Redirect {
// //         let cookie_id = super::authenticator::cookie_id();

// //         cookies.add_private(Cookie::new(cookie_id, self.get_authenticator().user().to_string()));
// //         Redirect::to(url.into().to_string())
// //     }

// //     /// Generates a failed response
// //     fn failure<T: Into<String>>(self, url: T) -> Redirect {
// //         Redirect::to(url.into().to_string())
// //     }

// //     /// Generates a failed response
// //     fn flash_failure<T: Into<String>, S: Into<String>>(self, url: T, msg: S) -> Flash<Redirect> {
// //         Flash::error(Redirect::to(url.into().to_string()), msg.into())
// //     }

// //     /// Generate an appropriate response based on the login status that the authenticator returned
// //     pub fn redirect<T: Into<String>, S: Into<String>>(self, success_url: T, failure_url: S, cookies: Cookies) -> Redirect {
// //         match self {
// //           Login::Success(_) => self.success(success_url, cookies),
// //           Login::Failure(_) => self.failure(failure_url)
// //         }
// //     }

// //     /// Generate an appropriate response based on the login status that the authenticator returned
// //     pub fn flash_redirect<T: Into<String>, S: Into<String>, R: Into<String>>(self, success_url: T, failure_url: S, failure_msg: R, cookies: Cookies) -> Result<Redirect,Flash<Redirect>> {
// //         match self {
// //           Login::Success(_) => Ok(self.success(success_url, cookies)),
// //           Login::Failure(_) => Err(self.flash_failure(failure_url, failure_msg))
// //         }
// //     }
// // }

// // impl<'f,A: Authenticator> FromForm<'f> for Login<A> {
// //     type Error = &'static str;
    
// //     fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
// //         let mut user_pass = HashMap::new();

// //         for form_item in form_items {
// //             let (key, value) = form_item.key_value_decoded();
// //             match key.as_str() {
// //                 "username" => user_pass.insert("username", value).map_or((), |_v| ()),
// //                 "password" => user_pass.insert("password", value).map_or((), |_v| ()),
// //                 _ => ()
// //             }
// //         }

// //         if user_pass.get("username").is_none() || user_pass.get("password").is_none() {
// //             Err("invalid form")
// //         } else {
// //             //TODO: find a way to get a DBConnection at this point
// //             let result = A::try_login(user_pass["username"].to_string(), user_pass["password"].to_string());

// //             Ok(match result {
// //                 Ok(authenticator) => Login::Success(authenticator),
// //                 Err(authenticator) => Login::Failure(authenticator)
// //             })
// //         }
// //     }
// // }

// #[derive(Debug)]
// pub enum LoginForm<A> {
//     Success(A),
//     Failure(A)
// }

// impl<A: Authenticator> LoginForm<A> {
//     #[inline(always)]
//     pub fn into_inner(self) -> A {
//         //self.0
//         match self {
//           LoginForm::Success(a) => a,
//           LoginForm::Failure(a) => a
//         }
//     }
// }

// impl<A: Authenticator> Deref for LoginForm<A> {
//     type Target = A;

//     fn deref(&self) -> &A {
//         //&self.0
//         match self {
//           LoginForm::Success(ref a) => a,
//           LoginForm::Failure(ref a) => a
//         }
//     }
// }

// impl<'f, A: Authenticator+FromForm<'f>> LoginForm<A> {
//     crate fn from_login_data(form_str: &'f str, strict: bool) -> Outcome<A, FormDataError<'f, <A as FromForm>::Error>> {
//         use self::FormDataError::*;

//         let mut items = FormItems::from(form_str);

//         // type Auth = (String, String);
//         // let mut user_pass: Auth;
//         // for form_item in items {
//         //     let (key, value) = form_item.key_value_decoded();
//         //     match key.as_str() {
//         //         "username" => user_pass.0 = value,
//         //         "password" => user_pass.1 = value,
//         //         _ => ()
//         //     }
//         // }
//         // if user_pass.0.is_empty() || user_pass.1.is_empty() {
//         //     error_!("Invalid login form with missing fiel 'username' or 'password'.");
//         //     return Failure((Status::UnprocessableEntity, Malformed(form_str)));
//         // }

//         let result = A::from_form(&mut items, strict);
//         if !items.exhaust() {
//             error_!("The request's form string was malformed.");
//             return Failure((Status::BadRequest, Malformed(form_str)));
//         }

//         match result {
//             Ok(v) => Success(v),
//             Err(e) => {
//                 error_!("The incoming form failed to parse.");
//                 Failure((Status::UnprocessableEntity, Parse(e, form_str)))
//             }
//         }
//     }

//     crate fn from_login_form(request: &Request<'_>, form_str: &'f str, strict: bool) -> Outcome<A, FormDataError<'f, <A as FromForm<'f>>::Error>> {
//         use self::FormDataError::*;

//         let mut items = FormItems::from(form_str);

//         // type Auth = (String, String);
//         // let mut user_pass: Auth;
//         // for form_item in items {
//         //     let (key, value) = form_item.key_value_decoded();
//         //     match key.as_str() {
//         //         "username" => user_pass.0 = value,
//         //         "password" => user_pass.1 = value,
//         //         _ => ()
//         //     }
//         // }
//         // if user_pass.0.is_empty() || user_pass.1.is_empty() {
//         //     error_!("Invalid login form with missing field 'username' or 'password'.");
//         //     return Failure((Status::UnprocessableEntity, Malformed(form_str)));
//         // }

//         let result = A::try_login(request, &mut items, strict);
//         if !items.exhaust() {
//             error_!("The request's form string was malformed.");
//             return Failure((Status::BadRequest, Malformed(form_str)));
//         }

//         match result {
//             Ok(v) => Success(v),
//             Err(e) => {
//                 error_!("The requested user was not found or wrong password was given.");
//                 Failure((Status::Unauthorized, Parse(e, form_str)))
//             }
//         }
//     }

//     /// Generates a succeed response
//     fn success<T: Into<String>>(self, url: T, mut cookies: Cookies) -> Redirect {
//         let cookie_id = super::authenticator::cookie_id();

//         cookies.add_private(Cookie::new(cookie_id, self.into_inner().user_token()));
//         Redirect::to(url.into().to_string())
//     }

//     /// Generates a failed response
//     fn failure<T: Into<String>>(self, url: T) -> Redirect {
//         Redirect::to(url.into().to_string())
//     }

//     /// Generates a failed response
//     fn flash_failure<T: Into<String>, S: Into<String>>(self, url: T, msg: S) -> Flash<Redirect> {
//         Flash::error(Redirect::to(url.into().to_string()), msg.into())
//     }

//     /// Generate an appropriate response based on the login status that the authenticator returned
//     pub fn redirect<T: Into<String>, S: Into<String>>(self, success_url: T, failure_url: S, cookies: Cookies) -> Redirect {
//         match self {
//           LoginForm::Success(_) => self.success(success_url, cookies),
//           LoginForm::Failure(_) => self.failure(failure_url)
//         }
//     }

//     /// Generate an appropriate response based on the login status that the authenticator returned
//     pub fn flash_redirect<T: Into<String>, S: Into<String>, R: Into<String>>(self, success_url: T, failure_url: S, failure_msg: R, cookies: Cookies) -> Result<Redirect,Flash<Redirect>> {
//         match self {
//           LoginForm::Success(_) => Ok(self.success(success_url, cookies)),
//           LoginForm::Failure(_) => Err(self.flash_failure(failure_url, failure_msg))
//         }
//     }
// }

// impl<'f, A: Authenticator+FromForm<'f>> FromData<'f> for LoginForm<A> {
//     type Error = FormDataError<'f, <A as FromForm<'f>>::Error>;
//     type Owned = String;
//     type Borrowed = str;

//     fn transform(
//         request: &Request,
//         data: Data
//     ) -> Transform<Outcome<Self::Owned, Self::Error>> {
//         use std::{cmp::min, io::Read};

//         let outcome = 'o: {
//             if !request.content_type().map_or(false, |ct| ct.is_form()) {
//                 warn_!("Form data does not have form content type.");
//                 break 'o Forward(data);
//             }
//             //TODO: uncomment when this gets integrated into Rocket
//             let limit = 4096;//request.limits().forms;
//             let mut stream = data.open().take(limit);
//             let mut form_string = String::with_capacity(min(4096, limit) as usize);
//             if let Err(e) = stream.read_to_string(&mut form_string) {
//                 break 'o Failure((Status::InternalServerError, FormDataError::Io(e)));
//             }

//             break 'o Success(form_string);
//         };

//         Transform::Borrowed(outcome)
//     }

//     fn from_data(req: &Request, o: Transformed<'f, Self>) -> Outcome<Self, Self::Error> {
//         <LoginForm<A>>::from_login_form(req, o.borrowed()?, true).map(LoginForm::Success)
//     }
// }

// impl<'f, A, T: FromUriParam<Query, A> + FromForm<'f>> FromUriParam<Query, A> for LoginForm<T> {
//     type Target = T::Target;

//     #[inline(always)]
//     fn from_uri_param(param: A) -> Self::Target {
//         T::from_uri_param(param)
//     }
// }