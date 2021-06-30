use rocket_dyn_templates::Template;
use rocket::http::ContentType;
use rocket::Request;
use std::fs;

#[derive(serde::Serialize)]
pub struct ErrorContext {

    error_code: &'static str,
    page: String,

    title: &'static str

}

//Helper Functions
fn read_file(path: String) -> std::fs::File { return fs::File::open(path).expect("Failed to open file") }

#[get("/css/pages/error.css")]
pub fn error_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("src/css/pages/error.css".to_string())) }

#[catch(404)]
pub fn not_found(req: &Request) -> Template {

    Template::render("pages/error", &ErrorContext {

        error_code: "404",
        page: req.uri().to_string(),

        title: "404"
    })

}

#[catch(500)]
pub fn my_prob(req: &Request) -> Template {

    Template::render("pages/error", &ErrorContext {

        error_code: "500",
        page: req.uri().to_string(),

        title: "500"
    })

}

