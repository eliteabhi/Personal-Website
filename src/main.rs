#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;

use rocket::Request;
use rocket_dyn_templates::Template;
use rocket::http::ContentType;
// use rocket::response::Redirect;

#[launch]
fn rocket() -> _ { // Launch website

    rocket::build()
                .attach(Template::fairing())
                .register("/", catchers![not_found])
                .mount("/", routes![index, css])

}



// Structs/Contexts
#[derive(serde::Serialize)]
struct IndexContext {
    date: &'static str,
    name: &'static str,

    parent: &'static str
}

#[derive(serde::Serialize)]
struct ErrorContext {

    error_code: &'static str,
    page: String,

    parent: &'static str

}

// CSS
#[get("/css/index.css")]
fn css() -> (ContentType, &'static str) {

    (ContentType::CSS, std::include_str!("../css/index.css"))

}



// Index(Default) page
#[get("/")]
fn index() -> Template {

    Template::render("index", &IndexContext {

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        parent: "layout"
    })
    
}



// Error handling
#[catch(404)]
fn not_found(req: &Request) -> Template {

    Template::render("error", &ErrorContext {

        error_code: "404",
        page: req.uri().to_string(),

        parent: "layout"
    })

}
