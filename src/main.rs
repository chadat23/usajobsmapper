#[macro_use]extern crate rocket;

mod search;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"See <a href="tera">Tera</a> or <a href="search">Search</a>."#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/search", routes![search::index, search::hello, search::about])
        .register("/search", catchers![search::not_found])
        .attach(Template::custom(|engines| {
            search::customize(&mut engines.handlebars);
        }))
}