use rocket_dyn_templates::Template;
use rocket::http::ContentType;
use std::fs;

//Helper Functions
pub fn read_file(path: String) -> std::fs::File { return fs::File::open(path).expect("Failed to open file") }

#[get("/css/pages/index.css")]
pub fn index_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("src/css/pages/index.css".to_string())) }


#[derive(serde::Serialize)]
pub struct IndexContext {

    date: &'static str,
    name: &'static str,

    title: &'static str

}

#[get("/")]
pub fn index() -> Template {

    Template::render("pages/index", &IndexContext {

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        title: "Home"
    })
    
}

