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

impl Location {
    fn full_name(&self) -> String {
        format!("{}, {}", self.city, self.state)
    }
}

#[derive(Debug)]
pub struct UsaJobsCredentials {
    pub apikey: String,
    pub useragent: String,
}

fn full_to_abrev() -> HashMap<String, String> {
    HashMap::from([
        (String::from("alabama"), String::from("al")),
        (String::from("alaska"), String::from("ak")),
        (String::from("arizona"), String::from("az")),
        (String::from("arkansas"), String::from("ar")),
        (String::from("california"), String::from("ca")),
        (String::from("colorado"), String::from("co")),
        (String::from("connecticut"), String::from("ct")),
        (String::from("delaware"), String::from("de")),
        (String::from("florida"), String::from("fl")),
        (String::from("georgia"), String::from("ga")),
        (String::from("hawaii"), String::from("hi")),
        (String::from("idaho"), String::from("id")),
        (String::from("illinois"), String::from("il")),
        (String::from("indiana"), String::from("in")),
        (String::from("iowa"), String::from("ia")),
        (String::from("kansas"), String::from("ks")),
        (String::from("kentucky"), String::from("ky")),
        (String::from("louisiana"), String::from("la")),
        (String::from("maine"), String::from("me")),
        (String::from("maryland"), String::from("md")),
        (String::from("massachusetts"), String::from("ma")),
        (String::from("michigan"), String::from("mi")),
        (String::from("minnesota"), String::from("mn")),
        (String::from("mississippi"), String::from("ms")),
        (String::from("missouri"), String::from("mo")),
        (String::from("montana"), String::from("mt")),
        (String::from("nebraska"), String::from("ne")),
        (String::from("nevada"), String::from("nv")),
        (String::from("new hampshire"), String::from("nh")),
        (String::from("new jersey"), String::from("nj")),
        (String::from("new mexico"), String::from("nm")),
        (String::from("new york"), String::from("ny")),
        (String::from("north carolina"), String::from("nc")),
        (String::from("north dakota"), String::from("nd")),
        (String::from("ohio"), String::from("oh")),
        (String::from("oklahoma"), String::from("ok")),
        (String::from("oregon"), String::from("or")),
        (String::from("pennsylvania"), String::from("pa")),
        (String::from("rhode island"), String::from("ri")),
        (String::from("south carolina"), String::from("sc")),
        (String::from("south dakota"), String::from("sd")),
        (String::from("tennessee"), String::from("tn")),
        (String::from("texas"), String::from("tx")),
        (String::from("utah"), String::from("ut")),
        (String::from("vermont"), String::from("vt")),
        (String::from("virginia"), String::from("va")),
        (String::from("washington"), String::from("wa")),
        (String::from("west virginia"), String::from("wv")),
        (String::from("wisconsin"), String::from("wi")),
        (String::from("wyoming"), String::from("wy")),
    ])
}

fn abrev_to_full() -> HashMap<String, String> {
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
    ])
}

pub fn make_locations(file: &str) -> HashMap<String, (String, String)> {
    let file = File::open(file).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut locations: HashMap<String, (String, String)> = HashMap::new();


    for line in reader.lines() {
        let line = line.unwrap().to_lowercase();

        match parse_location(&line) {
            Some(location) => { 
                locations.insert(location.full_name(), (location.lat, location.long));
            },
            None => {},            
        }

    }

    locations
}

fn parse_location(line: &str) -> Option<Location> {
    let mut line= line.split("\t");

    let city = String::from(line.nth(1).unwrap().trim());
    let lat = String::from(line.nth(2).unwrap().trim());
    let long = String::from(line.next().unwrap().trim());
    let mut state = String::from(line.nth(2).unwrap().trim());

    while state == String::from("") || state == String::from("us") {
        state = String::from(line.next().unwrap().trim());
    }

    match abrev_to_full().get(&state).cloned() {
        Some(state) => {
            Some(Location { city, lat, long, state })
        },
        None => None,
    }
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