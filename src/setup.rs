use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use figment::{Figment, providers::{Format, Toml, }};

#[derive(Debug)]
pub struct Place {
    name: String,
    lat: String,
    long: String,
}

#[derive(Debug)]
pub struct UsaJobsCredentials {
    pub apikey: String,
    pub useragent: String,
}

pub fn make_locations(files: [&str; 2]) -> HashMap<String, (String, String)> {
    let mut locations: HashMap<String, (String, String)> = HashMap::new();
    for file in files {
        let file = File::open(file).unwrap();
        let reader: BufReader<File> = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap().to_lowercase();

            let location = parse_location(&line);
            locations.insert(location.name, (location.lat, location.long));
        }
    }

    locations
}

fn parse_location(line: &str) -> Place {
    let mut line= line.split("\t");

    let name = String::from(line.next().unwrap().trim());
    let lat = String::from(line.next().unwrap().trim());
    let long = String::from(line.next().unwrap().trim());

    Place { name, lat, long }
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