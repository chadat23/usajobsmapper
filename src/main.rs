#[macro_use]extern crate rocket;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::io;
use std::collections::HashMap;

use figment::{Figment, providers::{Format, Toml, }};

use rocket::{fs::FileServer, futures::AsyncBufReadExt, };
use rocket::response::Redirect;
// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;
use search::UsaJobsCredentials;

mod search;

// #[get("/")]
// fn index() -> RawHtml<&'static str> {
//     RawHtml(r#"See <a href="tera">Tera</a> or <a href="search">Search</a>."#)
// }

#[derive(Debug)]
struct Location {
    city: String,
    lat: String,
    long: String,
    state: String,
}

fn make_locations(file: &str) -> HashMap<String, (String, String)> {
    // http://download.geonames.org/export/dump/

    let file = File::open(file).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut locations: HashMap<String, (String, String)> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut line= line.split("\t");
        let line = Location{
            city: String::from(line.nth(1).unwrap().trim()),
            lat: String::from(line.nth(2).unwrap().trim()),
            long: String::from(line.next().unwrap().trim()),
            state: String::from(line.nth(3).unwrap().trim()),
        };

        locations.insert(line.city, (line.lat, line.long));
    }

    locations
}

fn make_usajobs_credentials(file: &str) -> UsaJobsCredentials{
    let figment = rocket::Config::figment()
        .merge(Toml::file(file));

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
        .manage(make_usajobs_credentials("config.toml"))
        .manage(make_locations("US copy.txt"))
        .mount("/", routes![index])
        .mount("/search", routes![search::index, search::notes, search::about, search::search])
        .mount("/static", FileServer::from("static/"))
        .register("/search", catchers![search::not_found])
        .attach(Template::custom(|engines| {
            search::customize(&mut engines.handlebars);
        }))
}