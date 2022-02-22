use std::collections::HashMap;
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
    location_name: &'v str,
    radius: &'v str,
    continental_us: bool,
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
        payload = format!("{}JobCategoryCode={}&", payload, query.job_category_code.replace("\"", "").replace(" ", ";"))
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
    payload = format!("{}ResultsPerPage=2&", payload);
    payload = format!("{}Fields=Min&", payload);
    payload = format!("{}Page={}", payload, query.page);

    payload.replace(" ", "%20")
}