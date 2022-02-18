#[macro_use]extern crate rocket;

use rocket::{fs::FileServer, };
use rocket::response::Redirect;
// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

mod search;


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
    // rocket::custom(figment)
    //     .mount("/", routes![index])
    //     .mount("/search", routes![search::index, search::notes, search::about, search::search])
    //     .mount("/static", FileServer::from("static/"))
    //     .register("/search", catchers![search::not_found])
    //     .attach(Template::custom(|engines| {
    //         search::customize(&mut engines.handlebars);
    //     }))

    rocket::build()
        .mount("/", routes![index])
        .mount("/search", routes![search::index, search::notes, search::about, search::search])
        .mount("/static", FileServer::from("static/"))
        .register("/search", catchers![search::not_found])
        .attach(Template::custom(|engines| {
            search::customize(&mut engines.handlebars);
        }))
}