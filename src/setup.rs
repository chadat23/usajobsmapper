use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use figment::{Figment, providers::{Format, Toml, }};

// https://www.geonames.org/
// http://download.geonames.org/export/dump/
// https://www.gps-coordinates.net/
pub fn abrev_to_full() -> HashMap<String, String> {
    HashMap::from([
        (String::from("al"), String::from("alabama")),
        (String::from("ak"), String::from("alaska")),
        (String::from("az"), String::from("arizona")),
        (String::from("ar"), String::from("arkansas")),
        (String::from("ca"), String::from("california")),
        (String::from("co"), String::from("colorado")),
        (String::from("ct"), String::from("connecticut")),
        (String::from("de"), String::from("delaware")),
        (String::from("fl"), String::from("florida")),
        (String::from("ga"), String::from("georgia")),
        (String::from("hi"), String::from("hawaii")),
        (String::from("id"), String::from("idaho")),
        (String::from("il"), String::from("illinois")),
        (String::from("in"), String::from("indiana")),
        (String::from("ia"), String::from("iowa")),
        (String::from("ks"), String::from("kansas")),
        (String::from("ky"), String::from("kentucky")),
        (String::from("la"), String::from("louisiana")),
        (String::from("me"), String::from("maine")),
        (String::from("md"), String::from("maryland")),
        (String::from("ma"), String::from("massachusetts")),
        (String::from("mi"), String::from("michigan")),
        (String::from("mn"), String::from("minnesota")),
        (String::from("ms"), String::from("mississippi")),
        (String::from("mo"), String::from("missouri")),
        (String::from("mt"), String::from("montana")),
        (String::from("ne"), String::from("nebraska")),
        (String::from("nv"), String::from("nevada")),
        (String::from("nh"), String::from("new hampshire")),
        (String::from("nj"), String::from("new jersey")),
        (String::from("nm"), String::from("new mexico")),
        (String::from("ny"), String::from("new york")),
        (String::from("nc"), String::from("north carolina")),
        (String::from("nd"), String::from("north dakota")),
        (String::from("oh"), String::from("ohio")),
        (String::from("ok"), String::from("oklahoma")),
        (String::from("or"), String::from("oregon")),
        (String::from("pa"), String::from("pennsylvania")),
        (String::from("ri"), String::from("rhode island")),
        (String::from("sc"), String::from("south carolina")),
        (String::from("sd"), String::from("south dakota")),
        (String::from("tn"), String::from("tennessee")),
        (String::from("tx"), String::from("texas")),
        (String::from("ut"), String::from("utah")),
        (String::from("vt"), String::from("vermont")),
        (String::from("va"), String::from("virginia")),
        (String::from("wa"), String::from("washington")),
        (String::from("wv"), String::from("west virginia")),
        (String::from("wi"), String::from("wisconsin")),
        (String::from("wy"), String::from("wyoming")),
        //  ------------
        (String::from("dc"), String::from("washington district of columbia")),
        (String::from("gu"), String::from("guam")),
        (String::from("vg"), String::from("british virgin islands")),
    ])
}

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
