#[macro_use] extern crate rocket;

mod hbs;

// #[cfg(test)] mod tests;

// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

// #[get("/")]
// fn index() -> RawHtml<&'static str> {
//     RawHtml(r#"See <a href="tera">Tera</a> or <a href="hbs">Handlebars</a>."#)
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .mount("/", routes![index])
        .mount("/hbs", routes![hbs::index, hbs::hello])
        // .register("/hbs", catchers![hbs::not_found])
        .attach(Template::custom(|engines| {
            hbs::customize(&mut engines.handlebars);
        }))
}