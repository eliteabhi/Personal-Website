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

#[launch]
fn rocket() -> _ { // Launch website

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
fn css() -> (ContentType, fs::File) {

    let file = fs::File::open("css/index.css").expect("Failed to open favicon");

    (ContentType::CSS, file)

}



// Default Favicon
#[get("/images/favicon.ico")]
fn favicon() -> (ContentType, fs::File) {

    let file = fs::File::open("images/favicon.ico").expect("Failed to open favicon");

    (ContentType::Icon, file)

}
