use rocket::Request;
// use rocket::response::Redirect;

use rocket_dyn_templates::{Template, handlebars, context};

use self::handlebars::{Handlebars, JsonRender};
// use rocket::response::content::RawHtml;


#[get("/")]
pub fn index() -> Template {
    Template::render("search/index", context! {
        parent: "search/base",
    })
}

// #[post("/")]
// pub fn search()

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