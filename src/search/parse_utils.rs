use std::collections::{ HashMap, HashSet };

use rocket::serde::{Serialize, Deserialize};
use rocket::State;

use crate::search::query::Query;

#[derive(Debug, Serialize)]
pub struct Location {
    name: String,
    latitude: String,
    longitude: String,
}

#[derive(Debug, Serialize)]
pub struct Position {
    title: String,
    url: String,
    locations: Vec<Location>,
    orginization: String,
    department: String,
    low_grade: String,
    high_grade: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    total_search_results: u32,
    current_page: u32,
    number_of_pages: u32,
    positions: Vec<Position>,
    total_returned_locations: usize,
    continental_us: bool,
}

pub fn parse_request_into_jobs(response: String, query: &Query) -> SearchResult {
    // let current_page = query.page.parse::<u32>().unwrap();

    let response = json::parse(response.as_str()).unwrap();

    let search_result = &response["SearchResult"];

    let search_result_count = &search_result["SearchResultCount"];
    let search_result_count_all = &search_result["SearchResultCountAll"];
    let search_result_items = &search_result["SearchResultItems"];

    let mut positions: Vec<Position> = Vec::with_capacity(search_result_count.pretty(1).parse::<usize>().unwrap());
    let mut position_set: HashSet<String> = HashSet::new();
    for item in search_result_items.members() {
        let matched_object_descriptor = &item["MatchedObjectDescriptor"];
        let details = &matched_object_descriptor["UserArea"]["Details"];

        let position_locations = &matched_object_descriptor["PositionLocation"];        
        let mut locations: Vec<Location> = Vec::with_capacity(position_locations.len());
        for location in position_locations.members() {
            let name = location["LocationName"].pretty(0).replace("\"", "");
            position_set.insert(name.clone());
            locations.push(Location {
                name,
                latitude: location["LocationName"].pretty(0).replace("\"", ""),
                longitude: location["LocationName"].pretty(0).replace("\"", ""),
            });
        }

        positions.push(Position {
            title: matched_object_descriptor["PositionTitle"].pretty(0).replace("\"", ""),
            url: matched_object_descriptor["PositionURI"].pretty(0).replace("\"", ""),
            locations,
            orginization: matched_object_descriptor["OrganizationName"].pretty(0).replace("\"", ""),
            department: matched_object_descriptor["DepartmentName"].pretty(0).replace("\"", ""),
            low_grade: details["LowGrade"].pretty(0).replace("\"", ""),
            high_grade: details["HighGrade"].pretty(0).replace("\"", ""),
        });
    };

    SearchResult {
        total_search_results: search_result_count_all.pretty(0).replace("\"", "").parse::<u32>().unwrap(),
        number_of_pages: search_result["UserArea"]["NumberOfPages"].pretty(0).replace("\"", "").parse::<u32>().unwrap(),
        total_returned_locations: position_set.len(),
        current_page: query.page.parse::<u32>().unwrap(),
        continental_us: query.continental_us,
        positions,
    }
}

pub fn update_lat_long(mut results: SearchResult, places: &State<HashMap<String, (String, String)>>) -> SearchResult {
    for position in &mut results.positions {
        for location in &mut position.locations {
            let name = location.name.to_lowercase();

            match places.get(&name).cloned() {
                Some((lat, long)) => {
                    location.latitude = lat;
                    location.longitude = long;
                },
                None => {},
            }
        }
    }

    println!("{:?}", results);

    results
}