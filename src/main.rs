#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
extern crate image;

use std::fs;
use rocket::Request;
use rocket_dyn_templates::Template;
use rocket::http::ContentType;
// use rocket::response::Redirect;

#[launch]
fn rocket() -> _ { // Launch website

    rocket::build()
                //Fairings
                .attach(Template::fairing())

                //Error catchers
                .register("/", catchers![not_found])

                //Pages
                .mount("/", routes![index, about, css, favicon])

}



// Structs/Contexts
#[derive(serde::Serialize)]
struct IndexContext {

    date: &'static str,
    name: &'static str,

    title: &'static str,
    parent: &'static str

}

#[derive(serde::Serialize)]
struct AboutContext {

    date: &'static str,
    name: &'static str,

    title: &'static str,
    parent: &'static str

}

#[derive(serde::Serialize)]
struct ErrorContext {

    error_code: &'static str,
    page: String,

    title: &'static str,
    parent: &'static str

}



// CSS
#[get("/css/index.css")]
fn css() -> (ContentType, fs::File) {

    let file = fs::File::open("css/index.css").expect("Failed to open favicon");

    (ContentType::CSS, file)

}



// Favicon
#[get("/images/favicon.ico")]
fn favicon() -> (ContentType, fs::File) {

    let file = fs::File::open("images/favicon.ico").expect("Failed to open favicon");

    (ContentType::Icon, file)

}



// Index(Default) page
#[get("/")]
fn index() -> Template {

    Template::render("pages/index", &IndexContext {

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        title: "Home",
        parent: "layout"
    })
    
}

// About page
#[get("/about")]
fn about() -> Template {

    Template::render("pages/about", &AboutContext {

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        title: "About",
        parent: "layout"

    })

}



// Error handling
#[catch(404)]
fn not_found(req: &Request) -> Template {

    Template::render("pages/error", &ErrorContext {

        error_code: "404",
        page: req.uri().to_string(),

        title: "404",
        parent: "layout"
    })

}
