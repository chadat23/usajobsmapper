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
    Keyword: &'v str,
    PositionTitle: &'v str,
    HiringPath: Vec<HiringPath>,
    PayGradeLow: &'v str,
    PayGradeHigh: &'v str,
    Organization: Vec<Organization>,
    JobCategoryCode: &'v str,
    RelocationIndicator: bool,
    PositionScheduleTypeCode: Vec<PositionScheduleTypeCode>,
    LocationName: &'v str,
    Radius: &'v str,
    ContinentalUS: bool,
    SortField: SortField,
    SortDirection: SortDirection,
}

#[derive(Debug, FromFormField, Serialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, FromFormField, Serialize)]
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

#[derive(Debug, FromFormField, Serialize)]
pub enum PositionScheduleTypeCode {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
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