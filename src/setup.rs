use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use figment::{Figment, providers::{Format, Toml, }};

#[derive(Debug)]
struct Location {
    city: String,
    lat: String,
    long: String,
    state: String,
}

#[derive(Debug)]
pub struct UsaJobsCredentials {
    pub apikey: String,
    pub useragent: String,
}

pub fn make_locations(file: &str) -> HashMap<String, (String, String)> {
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

pub fn make_usajobs_credentials(file: &str) -> UsaJobsCredentials{
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

    UsaJobsCredentials { apikey, useragent}
}