#[macro_use] extern crate rocket;

use rocket::form::Form;

// https://github.com/chadat23/usajobsmapper2/blob/main/usajobsmapper/search_utils.py
// https://rocket.rs/v0.5-rc/guide/requests/#forms
#[derive(FromForm)]
struct SearchCriteria<'r> {
    Keyword: &'r str,
    locaton: &'r str,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/todo", data = "<search_criteria>")]
fn search(search_criteria: Form<SearchCriteria<'_>>) { 

 }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}