#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
extern crate image;

mod pages_rust {
    pub mod index;
    pub mod about;
    pub mod error;
}

use pages_rust::*;
use std::fs;
use rocket_dyn_templates::Template;
use rocket::http::ContentType;
// use rocket::response::Redirect;


// Launch website
#[launch]
fn rocket() -> _ {

    rocket::build()
                //Fairings
                .attach(Template::fairing())

                //Error catchers on Root
                .register("/", catchers![error::not_found])

                //Pages
                .mount("/", routes![index::index, global_css, error::error_css, index::index_css, about::about_css, favicon]) //Global (index page)
                .mount("/about", routes![about::about]) // About page

}


//Helper Functions
pub fn read_file(path: String) -> std::fs::File { return fs::File::open(path).expect("Failed to open file") }


// Global CSS
#[get("/css/global.css")]
fn global_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("src/css/global.css".to_string())) }


// Default Favicon
#[get("/images/favicon.ico")]
fn favicon() -> (ContentType, fs::File) { return (ContentType::Icon, read_file("src/images/favicon.ico".to_string())) }
