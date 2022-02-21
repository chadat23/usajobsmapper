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

fn full_to_abrev() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("Alabama", "AL"),
        ("Alaska", "AK"),
        ("Arizona", "AZ"),
        ("Arkansas", "AR"),
        ("California", "CA"),
        ("Colorado", "CO"),
        ("Connecticut", "CT"),
        ("Delaware", "DE"),
        ("Florida", "FL"),
        ("Georgia", "GA"),
        ("Hawaii", "HI"),
        ("Idaho", "ID"),
        ("Illinois", "IL"),
        ("Indiana", "IN"),
        ("Iowa", "IA"),
        ("Kansas", "KS"),
        ("Kentucky", "KY"),
        ("Louisiana", "LA"),
        ("Maine", "ME"),
        ("Maryland", "MD"),
        ("Massachusetts", "MA"),
        ("Michigan", "MI"),
        ("Minnesota", "MN"),
        ("Mississippi", "MS"),
        ("Missouri", "MO"),
        ("Montana", "MT"),
        ("Nebraska", "NE"),
        ("Nevada", "NV"),
        ("New Hampshire", "NH"),
        ("New Jersey", "NJ"),
        ("New Mexico", "NM"),
        ("New York", "NY"),
        ("North Carolina", "NC"),
        ("North Dakota", "ND"),
        ("Ohio", "OH"),
        ("Oklahoma", "OK"),
        ("Oregon", "OR"),
        ("Pennsylvania", "PA"),
        ("Rhode Island", "RI"),
        ("South Carolina", "SC"),
        ("South Dakota", "SD"),
        ("Tennessee", "TN"),
        ("Texas", "TX"),
        ("Utah", "UT"),
        ("Vermont", "VT"),
        ("Virginia", "VA"),
        ("Washington", "WA"),
        ("West Virginia", "WV"),
        ("Wisconsin", "WI"),
        ("Wyoming", "WY"),
    ])
}

fn abrev_to_full() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("AL", "Alabama"),
        ("AK", "Alaska"),
        ("AZ", "Arizona"),
        ("AR", "Arkansas"),
        ("CA", "California"),
        ("CO", "Colorado"),
        ("CT", "Connecticut"),
        ("DE", "Delaware"),
        ("FL", "Florida"),
        ("GA", "Georgia"),
        ("HI", "Hawaii"),
        ("ID", "Idaho"),
        ("IL", "Illinois"),
        ("IN", "Indiana"),
        ("IA", "Iowa"),
        ("KS", "Kansas"),
        ("KY", "Kentucky"),
        ("LA", "Louisiana"),
        ("ME", "Maine"),
        ("MD", "Maryland"),
        ("MA", "Massachusetts"),
        ("MI", "Michigan"),
        ("MN", "Minnesota"),
        ("MS", "Mississippi"),
        ("MO", "Missouri"),
        ("MT", "Montana"),
        ("NE", "Nebraska"),
        ("NV", "Nevada"),
        ("NH", "New Hampshire"),
        ("NJ", "New Jersey"),
        ("NM", "New Mexico"),
        ("NY", "New York"),
        ("NC", "North Carolina"),
        ("ND", "North Dakota"),
        ("OH", "Ohio"),
        ("OK", "Oklahoma"),
        ("OR", "Oregon"),
        ("PA", "Pennsylvania"),
        ("RI", "Rhode Island"),
        ("SC", "South Carolina"),
        ("SD", "South Dakota"),
        ("TN", "Tennessee"),
        ("TX", "Texas"),
        ("UT", "Utah"),
        ("VT", "Vermont"),
        ("VA", "Virginia"),
        ("WA", "Washington"),
        ("WV", "West Virginia"),
        ("WI", "Wisconsin"),
        ("WY", "Wyoming"),
    ])
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