use rocket_dyn_templates::Template;
use rocket::Request;

#[derive(serde::Serialize)]
pub struct ErrorContext {

    error_code: &'static str,
    page: String,

    title: &'static str,
    parent: &'static str

}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {

    Template::render("pages/error", &ErrorContext {

        error_code: "404",
        page: req.uri().to_string(),

        title: "404",
        parent: "layout"
    })

}
