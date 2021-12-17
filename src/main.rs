#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
extern crate image;

mod pages_rust {
    pub mod index;
    pub mod about;
    pub mod error;
    pub mod load;
    pub mod contact;
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
                .register("/", catchers![error::not_found, error::my_prob])

                //Pages
                .mount("/", routes![index::index, global_css, error::error_css, index::index_css, about::about_css, load::load_css, contact::contact_css, favicon]) //Global

                .mount("/loading...", routes![load::load, load::load_animation, global_css, load::load_css, favicon]) //Initial loading page
                .mount("/about", routes![about::about, global_css, about::about_css, favicon]) // About page
                .mount("/contact", routes![contact::contact, global_css, contact::contact_css, favicon]) // Contact page

}


//Helper Functions
pub fn read_file(path: String) -> fs::File { return fs::File::open(path).expect("Failed to open file"); }


// Global CSS
#[get("/css/global.css")]
fn global_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("css/global.css".to_string())); }


// Default Favicon
#[get("/images/favicon.ico")]
fn favicon() -> (ContentType, fs::File) { return (ContentType::Icon, read_file("images/favicon.ico".to_string())); }
