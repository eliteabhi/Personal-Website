use rocket_dyn_templates::Template;
use rocket::http::ContentType;
use std::fs;

#[derive(serde::Serialize)]
pub struct LoadContext {

    test_code: &'static str,
    page: String,

    title: &'static str

}

//Helper Functions
fn read_file(path: String) -> fs::File { return fs::File::open(path).expect("Failed to open file") }

#[get("/css/pages/load.css")]
pub fn load_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("src/css/pages/load.css".to_string())) }


#[get("/")]
pub fn load() -> Template {

    Template::render("pages/load", LoadContext{

        test_code: "404",
        page: "Loading...".to_string(),

        title:"Loading..."

    })

}
