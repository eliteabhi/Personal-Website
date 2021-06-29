#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
extern crate image;

mod pages {
    
    pub mod index;
    pub mod about;
    pub mod error;
}

use pages::*;
use std::fs;
use rocket_dyn_templates::Template;
use rocket::http::ContentType;
// use rocket::response::Redirect;

//Helper Functions
fn read_file(path: String) -> std::fs::File { return fs::File::open(path).expect("Failed to open file") }


// Launch website
#[launch]
fn rocket() -> _ {

    rocket::build()
                //Fairings
                .attach(Template::fairing())

                //Error catchers on Root
                .register("/", catchers![error::not_found])

                //Pages
                .mount("/", routes![index::index, css, favicon]) //Global (index)
                .mount("/about", routes![about::about]) // About page

}


// Global CSS
#[get("/css/index.css")]
fn css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("css/index.css".to_string())) }


// Default Favicon
#[get("/images/favicon.ico")]
fn favicon() -> (ContentType, fs::File) { return (ContentType::Icon, read_file("images/favicon.ico".to_string())) }
