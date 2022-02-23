#[macro_use]extern crate rocket;

use rocket::{fs::FileServer, futures::AsyncBufReadExt, };
use rocket::response::Redirect;
// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

mod search;
mod setup;
use setup::{ make_locations, make_usajobs_credentials };

// #[get("/")]
// fn index() -> RawHtml<&'static str> {
//     RawHtml(r#"See <a href="tera">Tera</a> or <a href="search">Search</a>."#)
// }

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/search", search::index))
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .manage(make_usajobs_credentials("config.toml"))
        // .manage(make_locations("US copy.txt"))
        .manage(make_locations("US.txt"))
        .mount("/", routes![index])
        .mount("/search", routes![search::index, search::notes, search::about, search::search, search::locations])
        .mount("/static", FileServer::from("static/"))
        .register("/search", catchers![search::not_found])
        .attach(Template::custom(|engines| {
            search::customize(&mut engines.handlebars);
        }))
}