use rocket_dyn_templates::Template;
use rocket::http::ContentType;
use std::fs;
use chrono::{Datelike, Utc};

//Helper Functions
pub fn read_file(path: String) -> std::fs::File { return fs::File::open(path).expect("Failed to open file"); }

#[get("/css/pages/index.css")]
pub fn index_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("css/pages/index.css".to_string())); }


#[derive(serde::Serialize)]
pub struct IndexContext {

    date: String,
    name: &'static str,

    title: &'static str

}

#[get("/")]
pub fn index() -> Template {

    let now = Utc::now();

    Template::render("pages/index", &IndexContext {

        date: format!("{:02}-{:02}-{}",
            now.month(),
            now.day(),
            now.year()),
        name: "Abhi Rangarajan",

        title: "Home"
    })
    
}

