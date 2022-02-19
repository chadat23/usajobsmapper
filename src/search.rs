use std::fmt::format;
use std::fmt;

use rocket::Request;
// use rocket::response::Redirect;

use rocket_dyn_templates::{Template, handlebars, context};

use rocket::serde::{Serialize,};
use rocket::serde::json::{Value};
use figment::{Figment, providers::{Format, Toml, }};

use self::handlebars::{Handlebars, JsonRender};
// use rocket::response::content::RawHtml;

use rocket::form::{Form, Contextual, FromForm, FromFormField};

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
    // page: &'v str,
}

#[derive(Debug, FromFormField, Serialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
    PUBIC,
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

#[get("/")]
pub fn index() -> Template {
    Template::render("search/index", context! {
        parent: "search/base",
    })
}

#[post("/query", data = "<form>")]
pub fn search<'r>(form: Form<Contextual<'r, Query<'r>>>) -> Value {
    let figment = rocket::Config::figment()
        .merge(Toml::file("config.toml"));

    let apikey = figment.find_value("usajobs").unwrap()                             
                             .as_dict().unwrap()
                             .get("APIKEY").unwrap()
                             .as_str().unwrap();
    let useragent = figment.find_value("usajobs").unwrap()
                                .as_dict().unwrap()
                                .get("USERAGENT").unwrap()
                                .as_str().unwrap();
    let username = figment.find_value("geonames").unwrap()
                               .as_dict().unwrap()
                               .get("USERNAME").unwrap()
                               .as_str().unwrap();

    let query = match form.value {
        Some(ref submission) => {
            submission
        }
        None => {
            panic!("oops")
        }
        
    };

    let url = make_url(query);

    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission);
            Template::render("search/index", &form.context)
        }
        None => {
            println!(" not submission: {:#?}", "lkj");
            Template::render("search/index", &form.context)
        }
    };

    let sub = match form.value {
        Some(ref submission) => {
            submission
        }
        None => {
            panic!("oops")
        }
        
    };
    
    serde_json::json!(sub)
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render("search/index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
        // This special key tells handlebars which template is the parent.
        parent: "search/base",
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("search/about.html", context! {
        title: "About",
        parent: "search/base",
    })
}

#[get("/notes")]
pub fn notes() -> Template {
    Template::render("search/notes", context! {
        title: "Notes",
        parent: "search/base",
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("search/errors/404", context! {
        uri: req.uri()
    })
}

fn wow_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

pub fn customize(hbs: &mut Handlebars) {
    hbs.register_helper("wow", Box::new(wow_helper));
    hbs.register_template_string("search/about.html", r#"
        {{#*inline "page"}}

        <section id="about">
          <h1>About - Here's another page!</h1>
        </section>
        
        {{/inline}}
        {{~> (parent)~}}
    "#).expect("valid HBS template");
}

fn make_url(query: &Query) -> String {
    let HOST = "data.usajobs.gov";
    let BASE_URL = "https://data.usajobs.gov/api/search?";

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

    payload.replace(" ", "%20")
}