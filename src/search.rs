use std::collections::HashMap;

use rocket::form::{Form, Contextual, FromForm, FromFormField};
use rocket::Request;
use rocket::serde::json::{Value};
use rocket::State;

use rocket_dyn_templates::{Template, handlebars, context};

use self::handlebars::{Handlebars, JsonRender};
// use rocket::response::content::RawHtml;

mod query;
use crate::search::query::make_query;
use query::{ Query };

mod parse_utils;

use crate::search::parse_utils::{ parse_request_into_jobs, update_lat_long };
use crate::setup::{ Places, UsaJobsCredentials };

#[get("/")]
pub fn index() -> Template {
    Template::render("search/index", context! {
        parent: "search/base",
    })
}

#[post("/query", data = "<form>")]
pub async fn search<'r>(form: Form<Contextual<'r, Query<'r>>>, 
    usajobs_credentials: &State<UsaJobsCredentials>,
    places: &State<HashMap<String, (String, String)>>) -> Value {
    // let query: Query = parse_form(form);
    let query = match form.value {
        Some(ref submission) => {
            submission
        }
        None => {
            panic!("oops")
        }
    };

    let jobs_request = make_query(&query, usajobs_credentials);
    let jobs_request = jobs_request.await;//.replace("\"", "");
    
    let positions = parse_request_into_jobs(jobs_request, &query);

    let positions = update_lat_long(positions, places);

    println!("about to jadon /////////////// {:#?}", positions);
    
    serde_json::json!(positions)
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

// fn parse_form<'r>(form: Form<Contextual<'r, Query<'r>>>) -> Query {
//     match form.value {
//         Some(ref submission) => {
//             submission
//         }
//         None => {
//             panic!("oops")
//         }
//     }
// }

// "{\"LanguageCode\":\"EN\",\"SearchParameters\":{},\"SearchResult\":{\"SearchResultCount\":2,\"SearchResultCountAll\":36,\"SearchResultItems\":[{\"MatchedObjectId\":\"636291300\",\"MatchedObjectDescriptor\":{\"PositionID\":\"AFMC-11387039-526374-9V-WR\",\"PositionTitle\":\"LEAD FIREFIGHTER\",\"PositionURI\":\"https://www.usajobs.gov:443/GetJob/ViewDetails/636291300\",\"ApplyURI\":[\"https://www.usajobs.gov:443/GetJob/ViewDetails/636291300?PostingChannelID=\"],\"PositionLocationDisplay\":\"Tinker AFB, Oklahoma\",\"PositionLocation\":[{\"LocationName\":\"Tinker AFB, Oklahoma\",\"CountryCode\":\"United States\",\"CountrySubDivisionCode\":\"Oklahoma\",\"CityName\":\"Tinker AFB, Oklahoma\",\"Longitude\":-97.3894,\"Latitude\":35.422}],\"OrganizationName\":\"Air Force Materiel Command\",\"DepartmentName\":\"Department of the Air Force\",\"SubAgency\":\"72 ABW/CEFO, Announcement may be used to fill other positions at Tinker AFB.\",\"JobCategory\":[{\"Name\":\"Fire Protection and Prevention\",\"Code\":\"0081\"}],\"JobGrade\":[{\"Code\":\"GS\"}],\"PositionSchedule\":[{\"Name\":\"Full-time\",\"Code\":\"1\"}],\"PositionOfferingType\":[{\"Name\":\"Permanent\",\"Code\":\"15317\"}],\"QualificationSummary\":\"In order to qualify, you must meet the specialized experience requirements described in the Office of Personnel Management (OPM) Qualification Standards for General Schedule Positions, to include the Individual Occupational Requirements (IOR) for the GS-0081, Fire Protection and Prevention Series. SPECIALIZED EXPERIENCE: Applicants must have at least one (1) year, or will have one year within 120 days of closing of this announcement, specialized experience at a level close to the work of this position that has given them the particular knowledge, skills and abilities required to successfully perform. Typically, we would find this experience in work within this field that is closely related. To be creditable, one year of the specialized experience must have been equivalent to the next lower grade level (GS-07), pay band or equivalent. Specialized experience is described as (1) controlling or extinguishing fires as a member of an organized military, industrial, volunteer, or governmental fire department or brigade; (2) rescue operations; (3) detection, reduction, or elimination of potential fire hazards; (4) operation of fire communications equipment; (5) controlling hazardous materials incidents and/or (6) developing, implementing, or providing training in fire protection and prevention. FEDERAL TIME-IN-GRADE (TIG) REQUIREMENT FOR GENERAL SCHEDULE (GS) POSITIONS: Merit promotion applicants must meet applicable time-in-&shy;grade requirements to be considered eligible. One year at the GS-07 level is required to meet the time-in-grade requirements for the GS-08 level. TIG applies if you are in a current GS position or held a GS position within the previous 52 weeks. Due to the use of 120 day rosters, applicants within 120 days of completion of TIG requirements are encouraged to apply. KNOWLEDGE, SKILLS AND ABILITIES (KSAs): Your qualifications will be evaluated on the basis of your level of knowledge, skills, abilities and/or competencies in the following areas: 1. Knowledge to drive and operate firefighting vehicles of significant complexity; of the principles of hydraulics as they pertain to water flow, water pressure, water levels, line (friction) losses, etc.; and of basic and specialized firefighting equipment (fire alarm system operation, fire extinguishing equipment operation, etc.), techniques, and procedures.\\n2. Knowledge of emergency first aid techniques; safety requirements as outlined in applicable safety standards, regulations, and/or technical orders; and HAZMAT emergency response procedures, equipment, and identification.\\n3. Knowledge of basic building design, construction, and occupancy; and of the latest changes in aircraft cockpit design, ordnance placement, and cabin layouts to include the operation of hatch or canopy release mechanisms, ejection seat mechanisms, and/or oxygen supply systems.\\n4. Skill in detecting and recognizing fire hazards (potential and immediate), and operating communications equipment.\\n5. Ability to maintain good working relations; communicate orally and in writing; and lift and carry heavy loads. *Employee is required to be certified by the Department of Defense (DoD) Firefighter Certification System as Fire Officer I, Fire Instructor I, Fire Inspector I, HAZMAT Awareness, HAZMAT Operations, Airport Firefighter(includes Firefighter I, II), Apparatus Driver Operator- Aircraft Rescue and Firefighting, Apparatus Driver Operator- Pumper, Apparatus Driver Operator- Aerial*, Apparatus Driver Operator- Mobile Water Supply(MWS*). Please provide copies of certifications with application.*NOTE- Required if Aerial and/or MWS is assigned. MAXIMUM ENTRY AGE: Title 5 U.S.C. 3307 authorizes the head of any agency to establish a maximum entry age for the original appointment of individuals to the position of primary and rigorous firefighter. The date immediately preceding an individual's 37th birthday is the maximum entry age for original appointment to a position as a Firefighter. This age restriction does not apply to those who have previously served in a Federal civilian firefighting position covered by title 5 U.S.C. section 3307 provisions OR qualified veteran preference eligible covered by title 5 U.S.C. 3307. *Special Retirement Provisions Authority:\\n- FERS: Position covered as rigorous under the FERS special retirement provisions for Federal firefighters [5 U.S.C. 8401(14), 5 U.S.C. 8412(d), and 5 CFR 842.802]. Authority, Under Secretary of Defense Memo, dated 30 December 2004.\\n- CSRS: Position covered as rigorous under the CSRS special retirement provisions for Federal firefighters [5 U.S.C. 8331(21), 5 U.S.C. 8336(c), and 5 CFR 831.902]. Authority, Under Secretary of Defense Memo, dated 30 December 2004. This position has been designated as Key. In the event of a crisis, selectee must perform the Key duties until relieved. This position cannot be vacated during a national emergency or mobilization. Requires the incumbent be removed from military recall status. This is a Mission Essential (ME) position performing a Mission Critical Function (MCF) which must continue uninterrupted after the occurrence of an emergency and continued through full resumption of all functions. PART-TIME OR UNPAID EXPERIENCE: Credit will be given for appropriate unpaid and or part-time work. You must clearly identify the duties and responsibilities in each position held and the total number of hours per week. VOLUNTEER WORK EXPERIENCE: Refers to paid and unpaid experience, including volunteer work done through National Service Programs (i.e., Peace Corps, AmeriCorps) and other organizations (e.g., professional; philanthropic; religious; spiritual; community; student and social). Volunteer work helps build critical competencies, knowledge and skills that can provide valuable training and experience that translates directly to paid employment. You will receive credit for all qualifying experience, including volunteer experience.\",\"PositionRemuneration\":[{\"MinimumRange\":\"49549.0\",\"MaximumRange\":\"64410.0\",\"RateIntervalCode\":\"Per Year\"}],\"PositionStartDate\":\"2022-02-10T00:00:00.0000\",\"PositionEndDate\":\"2022-02-24T23:59:59.9970\",\"PublicationStartDate\":\"2022-02-10T00:00:00.0000\",\"ApplicationCloseDate\":\"2022-02-24T23:59:59.9970\",\"PositionFormattedDescription\":[{\"Label\":\"Dynamic Teaser\",\"LabelDescription\":\"Hit highlighting for keyword searches.\"}],\"UserArea\":{\"Details\":{\"JobSummary\":\"Click on Learn more about this agency button below to view Eligibilities being considered and other IMPORTANT information. The primary purpose of this position, Lead Firefighter, GS-0081-08, is to be responsible for a major piece of firefighting equipment and its regularly assigned crew of three or more firefighters engaged in airfield and structural firefighting and rescue operations and the reduction and/or elimination of potential fire hazards.\",\"WhoMayApply\":{\"Name\":\"\",\"Code\":\"\"},\"LowGrade\":\"8\",\"HighGrade\":\"8\",\"PromotionPotential\":\"8\",\"HiringPath\":[\"fed-transition\",\"overseas\",\"fed-competitive\",\"fed-excepted\",\"disability\",\"land\",\"mspouse\",\"peace\",\"vet\"],\"AgencyMarketingStatement\":\"The mission of the United States Air Force is: To fly, fight, and win ... Airpower anytime, anywhere. To achieve that mission, the Air Force has a vision of Global Vigilance, Global Reach and Global Power. That vision orbits around three core competencies: Developing Airmen, Technology-to-Warfighting and Integrating Operations. Core competencies and distinctive capabilities are based on a shared commitment to three core values -- integrity first, service before self, and excellence in all we do. Click here to view the AF Civilian Employment Eligibility Guide: 30 Percent or More Disabled VeteransAF DCIPS InterchangeAF Internal EmployeeDIBF or MRTFBDoD Transfer (Excluding Air Force)EO 12721 Certain Former Overseas EmployeesEO 13473 Appointment of Certain Military SpousesFormer Federal Employees (Reinstatement)Interagency Career Transition Assistance PlanLand Management EmployeeMilitary Spouse PreferenceNational Service (Peace Corps and VISTA)Non-AF DCIPS InterchangeNon-Appropriated FundNon-DoD TransferOther (Interchange Agreements)People with Disabilities, Schedule AVeterans Employment Opportunities Act\",\"TravelCode\":\"1\",\"ApplyOnlineUrl\":\"https://apply.usastaffing.gov/Application/Apply\",\"DetailStatusUrl\":\"https://apply.usastaffing.gov/Application/ApplicationStatus\"},\"IsRadialSearch\":false}},\"RelevanceRank\":0},{\"MatchedObjectId\":\"631894100\",\"MatchedObjectDescriptor\":{\"PositionID\":\"7Q-AFPC-11358258-486514-CCK\",\"PositionTitle\":\"FIRE PROTECTION INSPECTOR\",\"PositionURI\":\"https://www.usajobs.gov:443/GetJob/ViewDetails/631894100\",\"ApplyURI\":[\"https://www.usajobs.gov:443/GetJob/ViewDetails/631894100?PostingChannelID=\"],\"PositionLocationDisplay\":\"McConnell AFB, Kansas\",\"PositionLocation\":[{\"LocationName\":\"McConnell AFB, Kansas\",\"CountryCode\":\"United States\",\"CountrySubDivisionCode\":\"Kansas\",\"CityName\":\"McConnell AFB, Kansas\",\"Longitude\":-97.26118,\"Latitude\":37.622684}],\"OrganizationName\":\"Air Mobility Command\",\"DepartmentName\":\"Department of the Air Force\",\"JobCategory\":[{\"Name\":\"Fire Protection and Prevention\",\"Code\":\"0081\"}],\"JobGrade\":[{\"Code\":\"GS\"}],\"PositionSchedule\":[{\"Name\":\"Full-time\",\"Code\":\"1\"}],\"PositionOfferingType\":[{\"Name\":\"13 Months\",\"Code\":\"15319\"}],\"QualificationSummary\":\"In order to qualify, you must meet the specialized experience requirements described in the Office of Personnel Management (OPM) Qualification Standards for General Schedule Positions to include the Individual Occupational Requirements (IOR) for the GS-0081, Fire Protection and Prevention Series. SPECIALIZED EXPERIENCE: Applicants must have at least one (1) year of specialized experience at the next lower grade GS-07, or equivalent in other pay systems. Examples of specialized experience include: 1) Developing and conducting extensive fire protection and prevention education and training programs; 2) Providing materials, and situational and classroom training at installation fire prevention activities such as Fire Prevention Week, Open Houses, etc., and addresses a variety of client groups in fire program methods and goals.; 3) Locating, isolating and identifying actual or potential fire hazards and fire safety deficiencies and initiating appropriate corrective action; and 4) Participating with fire prevention or safety engineers in studying fire prevention problems and investigating actual incidents. NOTE: Your answer must be supported by information in your resume. FEDERAL TIME-IN-GRADE (TIG) REQUIREMENT FOR GENERAL SCHEDULE (GS) POSITIONS: Merit promotion applicants must meet applicable time-in-&shy;grade requirements to be considered eligible. One year at the GS-07 level is required to meet the time-in-grade requirements for the GS-08 level. TIG applies if you are in a current GS position or held a GS position within the previous 52 weeks. NOTE: Applicants applying as VEOA candidates who are current GS civil service employees or are prior GS civil service employees within the past 52 weeks must also meet time-in-grade requirements. Effective 1 Jun 00, all DoD firefighters and contract fire and emergency service personnel must be certified at the next higher level before being eligible for promotion to that level. Applicants must list their current firefighter related licenses and certificates on their resumes. Please attach copies of certifications when you submit the resume to verify qualification requirements for the position. Certification requirements for this position are: Department of Defense (DOD) Firefighter Certification Program as Airport Firefighter, Firefighter I, Firefighter II, Fire Inspector I &amp; II and Fire Instructor I, HAZMAT Awareness, and HAZMAT Operations. IMPORTANT: Please upload copies of your FF certs and EMR/EMT card. If you have received a Student ID# for the DoD Fire and Emergency Services Certification Program website (https://go.usa.gov/xdsTR) please provide a copy of your most recent certification transcript. Note: If you do not know your Student ID, you may contact the AFCEC Reachback center at afcec.rbc@us.af.mil. If you do not have a Student ID and/or cannot access the Certificate Program site, you must upload a copy of your individual certifications or transcript at the time of your application or you will be removed from consideration. Special Retirement Provisions Authority: FERS Position covered as secondary under the FERS special retirement provisions for Federal firefighters [5 U.S.C. 8401(14), 5 U.S.C. 8412(d), and 5 CFR 842.802]. Position covered as secondary under the CSRS special retirement provisions for Federal firefighters [5 U.S.C. 8331(21), 5 U.S.C. 8336(c), and 5 CFR 831.902]. KNOWLEDGE, SKILLS AND ABILITIES (KSAs): Your qualifications will be evaluated on the basis of your level of knowledge, skills, abilities and/or competencies in the following areas: 1. Knowledge of recognized standards covering protection and prevention techniques and procedures; agency and instillation manuals, rules and regulations; and federal, state, and local fire protection/prevention ordinances and building codes.\\n2. Knowledge of the theory of fire propagation and progression and the application of accepted techniques and principles to reduce/eliminate hazards in a specialized environment with a high fire expectancy and potential for severity\\n3. Knowledge of the layout, design, construction, and contents of buildings; locations of fire protection systems; and other similar characteristics of the installation; and of the latest changes in aircraft cockpit design, ordnance placement, and cabin layout including the operation of hatch or canopy release mechanisms, ejection seat mechanisms, and/or oxygen supply systems when work is associated with active aircraft.\\n4. Knowledge of health and environmental safety requirements as outlined in applicable safety standards, regulations, and/or technical orders.\\n5. Ability to develop and conduct training, education, and information programs for a variety of client groups; to maintain good working relations and to use tact and firmness when dealing with operating officials; and to communicate orally and in writing.\\n6. Ability to recognize new or unusual types of combustibles or other hazardous material, determine their fire expectancy and severity and adapt fire protection/prevention techniques to correct conditions or hazardous situations. PART-TIME OR UNPAID EXPERIENCE: Credit will be given for appropriate unpaid and or part-time work. You must clearly identify the duties and responsibilities in each position held and the total number of hours per week. VOLUNTEER WORK EXPERIENCE: Refers to paid and unpaid experience, including volunteer work done through National Service Programs (i.e., Peace Corps, AmeriCorps) and other organizations (e.g., professional; philanthropic; religious; spiritual; community; student and social). Volunteer work helps build critical competencies, knowledge and skills that can provide valuable training and experience that translates directly to paid employment. You will receive credit for all qualifying experience, including volunteer experience.\",\"PositionRemuneration\":[{\"MinimumRange\":\"49549.0\",\"MaximumRange\":\"64410.0\",\"RateIntervalCode\":\"Per Year\"}],\"PositionStartDate\":\"2022-01-19T00:00:00.0000\",\"PositionEndDate\":\"2022-02-21T23:59:59.9970\",\"PublicationStartDate\":\"2022-01-19T00:00:00.0000\",\"ApplicationCloseDate\":\"2022-02-21T23:59:59.9970\",\"PositionFormattedDescription\":[{\"Label\":\"Dynamic Teaser\",\"LabelDescription\":\"Hit highlighting for keyword searches.\"}],\"UserArea\":{\"Details\":{\"JobSummary\":\"Click on Learn more about this agency button below to view Eligibilities being considered and other IMPORTANT information. The primary purpose of this position is to perform fire prevention and inspection duties designed to detect and reduce or eliminate fire hazards in a high fire expectancy area. Develops and conducts extensive fire protection and prevention education and training programs.\",\"WhoMayApply\":{\"Name\":\"\",\"Code\":\"\"},\"LowGrade\":\"8\",\"HighGrade\":\"8\",\"PromotionPotential\":\"None\",\"HiringPath\":[\"fed-transition\",\"overseas\",\"fed-competitive\",\"fed-excepted\",\"disability\",\"mspouse\",\"vet\"],\"TotalOpenings\":\"Few\",\"AgencyMarketingStatement\":\"The mission of the United States Air Force is: To fly, fight, and win...Airpower anytime, anywhere. To achieve that mission, the Air Force has a vision of Global Vigilance, Global Reach and Global Power. That vision orbits around three core competencies: Developing Airmen, Technology-to-Warfighting and Integrating Operations. Core competencies and distinctive capabilities are based on a shared commitment to three core values -- integrity first, service before self, and excellence in all we do. Click here to view the AF Civilian Employment Eligibility Guide: 30 Percent or More Disabled VeteransAF Internal EmployeeDoD Transfer (Excluding Air Force)EO 12721 Certain Former Overseas EmployeesEO 13473 Appointment of Certain Military SpousesFormer Federal Employees (Reinstatement)Interagency Career Transition Assistance PlanMilitary Spouse PreferenceNon-DoD TransferPeople with Disabilities, Schedule AVeterans Recruitment Authority\",\"TravelCode\":\"0\",\"ApplyOnlineUrl\":\"https://apply.usastaffing.gov/Application/Apply\",\"DetailStatusUrl\":\"https://apply.usastaffing.gov/Application/ApplicationStatus\"},\"IsRadialSearch\":false}},\"RelevanceRank\":0}],\"UserArea\":{\"NumberOfPages\":\"18\",\"IsRadialSearch\":false}}}"