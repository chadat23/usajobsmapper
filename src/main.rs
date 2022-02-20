#[macro_use]extern crate rocket;

use figment::{Figment, providers::{Format, Toml, }};

use rocket::{fs::FileServer, };
use rocket::response::Redirect;
// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;
use search::UsaJobsCredentials;

mod search;

// #[get("/")]
// fn index() -> RawHtml<&'static str> {
//     RawHtml(r#"See <a href="tera">Tera</a> or <a href="search">Search</a>."#)
// }

fn make_usajobs_credentials() -> UsaJobsCredentials{
    let figment = rocket::Config::figment()
        .merge(Toml::file("config.toml"));

    let apikey = String::from(figment.find_value("usajobs").unwrap()                             
                             .as_dict().unwrap()
                             .get("APIKEY").unwrap()
                             .as_str().unwrap());
    let useragent = String::from(figment.find_value("usajobs").unwrap()
                                .as_dict().unwrap()
                                .get("USERAGENT").unwrap()
                                .as_str().unwrap());

    search::UsaJobsCredentials { apikey, useragent}
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/search", search::index))
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .manage(make_usajobs_credentials())
        .mount("/", routes![index])
        .mount("/search", routes![search::index, search::notes, search::about, search::search])
        .mount("/static", FileServer::from("static/"))
        .register("/search", catchers![search::not_found])
        .attach(Template::custom(|engines| {
            search::customize(&mut engines.handlebars);
        }))
}