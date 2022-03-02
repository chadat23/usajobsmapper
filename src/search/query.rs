use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fmt;
use std::str::FromStr;

use rocket::form::{Form, Contextual, FromForm, FromFormField};
use rocket::serde::{Serialize, Deserialize};
use rocket::State;

use crate::setup::UsaJobsCredentials;

pub async fn make_query<'r>(query: &Query<'_>, usajobs_credentials: &State<UsaJobsCredentials>) -> String {
    let url = make_url(query);

    let jobs_request = make_jobs_request(url, usajobs_credentials);
    jobs_request.await.text().await.unwrap()
}

#[derive(Debug, FromFormField, Serialize, PartialEq)]
pub enum SortField {
    DEFAULT,
    OPENDATE,
    CLOSEDATE,
    JOBTITLE,
    SALARY,
    LOCATION,
    DEPARTMENT,
    TITLE,
}

impl fmt::Display for SortField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = format!("{:?}", self);
        write!(f, "{}", text.to_lowercase())
    }
}

#[derive(Copy, Clone, Debug, FromFormField, Serialize)]
pub enum PositionScheduleTypeCode {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

impl fmt::Display for PositionScheduleTypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self as u32)
    }
}

#[derive(Debug, FromFormField, Serialize)]
pub enum Organization {
    AG,
    AF,                
    AR,                
    CM,
    FQ,
    DD,
    ED,
    DN,
    EOP,
    GS,
    HE,
    HS,
    HU,
    IN,
    JL,
    DJ,
    DL,
    LL,
    NN,
    AH,
    NV,
    OT,
    ST,
    TD,
    TR,
    VA,
}

impl fmt::Display for Organization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, FromFormField, Serialize)]
enum HiringPath {
    PUBLIC,
    VET,
    NGUARD,
    DISABILITY,
    NATIVE,
    MSPOUSE,
    STUDENT,
    SES,
    PEACE,
    OVERSEAS,
    FED_INTERNAL_SEARCH,
    GRADUATES,
    FED_EXCEPTED,
    FED_COMPETITIVE,
    FED_TRANSITION,
    LAND,
    SPECIAL_AUTHORITIES,
}

impl fmt::Display for HiringPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = format!("{:?}", self);
        write!(f, "{}", text.to_lowercase().replace("_", "-"))
    }
}

#[derive(Debug, FromForm, Serialize)]
pub struct Query<'v> {
    keyword: &'v str,
    position_title: &'v str,
    hiring_path: Vec<HiringPath>,
    pay_grade_low: &'v str,
    pay_grade_high: &'v str,
    organization: Vec<Organization>,
    job_category_code: &'v str,
    relocation_indicator: bool,
    position_schedule_type_code: Vec<PositionScheduleTypeCode>,
    pub location_name: &'v str,
    pub radius: &'v str,
    pub continental_us: bool,
    sort_field: SortField,
    sort_direction: SortDirection,
    pub page: &'v str,
}

#[derive(Debug, FromFormField, Serialize,)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

async fn make_jobs_request(url: String, config: &State<UsaJobsCredentials>) -> reqwest::Response {
    const HOST: &str = "data.usajobs.gov";

    let client = reqwest::Client::new();

    let req = client
        .get(url)
        .header("Host", HOST)
        .header("User-Agent", config.useragent.as_str())
        .header("Authorization-Key", config.apikey.as_str());

    req.send().await.unwrap()
}

fn make_url(query: &Query) -> String {
    const BASE_URL: &str = "https://data.usajobs.gov/api/search?";

    let mut payload: String = String::from(BASE_URL);

    if query.keyword != "" {
        payload = format!("{}Keyword={}&", payload, query.keyword)
    }
    if query.position_title != "" {
        payload = format!("{}PositionTitle={}&", payload, query.position_title)
    }
    if !query.hiring_path.is_empty() {
        payload = format!("{}HiringPath=", payload);

        let mut hiring_paths = query.hiring_path.iter();
        while hiring_paths.len() > 0 {
            payload = format!("{}{}", payload, hiring_paths.next().unwrap());
            if hiring_paths.len() > 0 {               
                payload = format!("{}{}", payload, ";");
            }
        }
        payload = format!("{}&", payload);
    }
    if query.pay_grade_low != "" {
        payload = format!("{}PayGradeLow={}&", payload, query.pay_grade_low)
    }
    if query.pay_grade_high != "" {
        payload = format!("{}PayGradeHigh={}&", payload, query.pay_grade_high)
    }
    if !query.organization.is_empty() {
        payload = format!("{}Organization=", payload);

        let mut organizations = query.organization.iter();
        while organizations.len() > 0 {
            payload = format!("{}{}", payload, organizations.next().unwrap());
            if organizations.len() > 0 {               
                payload = format!("{}{}", payload, ";");
            }
        }
        payload = format!("{}&", payload);
    }
    if query.job_category_code != "" {
        payload = format!("{}JobCategoryCode={}&", payload, query.job_category_code.replace("\"", "").replace(";", "").replace(",", "").replace(" ", ";"))
    }
    if query.relocation_indicator {
        payload = format!("{}RelocationIndicator=True&", payload);
    }
    if !query.position_schedule_type_code.is_empty() {
        payload = format!("{}PositionScheduleTypeCode=", payload);

        let mut position_schedule_type_codes = query.position_schedule_type_code.iter();
        while position_schedule_type_codes.len() > 0 {
            payload = format!("{}{}", payload, position_schedule_type_codes.next().unwrap());
            if position_schedule_type_codes.len() > 0 {               
                payload = format!("{}{}", payload, ";");
            }
        }
        payload = format!("{}&", payload);
    }
    if query.location_name != "" {
        payload = format!("{}LocationName={}&", payload, query.location_name)
    }
    if query.radius != "" {
        payload = format!("{}Radius={}&", payload, query.radius)
    }
    if query.sort_field != SortField::DEFAULT {
        payload = format!("{}SortField={}&", payload, query.sort_field)
    }
    payload = format!("{}SortDirection={}&", payload, query.sort_direction);
    payload = format!("{}ResultsPerPage=10&", payload);
    payload = format!("{}Fields=Min&", payload);
    payload = format!("{}Page={}", payload, query.page);

    payload.replace(" ", "%20")
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub name: String,
    pub latitude: String,
    pub longitude: String,
    pub found: bool,
}

#[derive(Debug, Serialize)]
pub struct Position {
    pub title: String,
    pub url: String,
    pub id: String,
    pub locations: Vec<Location>,
    pub orginization: String,
    pub department: String,
    pub low_grade: String,
    pub high_grade: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub total_search_results: u32,
    pub current_page: u32,
    pub number_of_pages: u32,
    pub positions: Vec<Position>,
    pub total_returned_locations: usize,
    pub continental_us: bool,
    pub radius: u32,
    pub radius_center:[f32; 2],
}

impl SearchResult {
    pub fn from_response_and_query(response: String, query: &Query,
        places: &State<HashMap<String, (String, String)>>, 
        states: &State<HashMap<String, String>>) -> Self {
        // https://developer.usajobs.gov/API-Reference/GET-api-Search
    
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

                let location_info = match places.get(&name.to_lowercase()).cloned() {
                    Some((lat, long)) => (lat, long, true),
                    None => ((39.833333).to_string(), (-98.583333).to_string(), false),
                };
                locations.push(Location {
                    name,
                    latitude: location_info.0,
                    longitude: location_info.1,
                    found: location_info.2,
                });
            }
    
            positions.push(Position {
                title: matched_object_descriptor["PositionTitle"].pretty(0).replace("\"", ""),
                url: matched_object_descriptor["PositionURI"].pretty(0).replace("\"", ""),
                id: matched_object_descriptor["PositionID"].pretty(0).replace("\"", ""),
                locations,
                orginization: matched_object_descriptor["OrganizationName"].pretty(0).replace("\"", ""),
                department: matched_object_descriptor["DepartmentName"].pretty(0).replace("\"", ""),
                low_grade: details["LowGrade"].pretty(0).replace("\"", ""),
                high_grade: details["HighGrade"].pretty(0).replace("\"", ""),
            });
        };

        let radius_center = match states.get(query.location_name.split(", ").last().unwrap()) {
            Some(full_state) => {
                let chunk = query.location_name.split(", ").last().unwrap();
                let radius_center = query.location_name.replace(chunk, full_state);
                radius_center
            },
            None => query.location_name.to_string(),
        };

        let radius_center = match places.get(radius_center.as_str()) {
            Some(center) => {
                [center.0.parse::<f32>().unwrap(), center.1.parse::<f32>().unwrap()]
            },
            None => {
                [0.0, 0.0]
            }
        };
    
        SearchResult {
            total_search_results: search_result_count_all.pretty(0).replace("\"", "").parse::<u32>().unwrap(),
            number_of_pages: search_result["UserArea"]["NumberOfPages"].pretty(0).replace("\"", "").parse::<u32>().unwrap(),
            total_returned_locations: position_set.len(),
            current_page: query.page.parse::<u32>().unwrap(),
            continental_us: query.continental_us,
            positions,
            radius: query.radius.parse::<u32>().unwrap_or(0),
            radius_center: radius_center,
        }
    }
}