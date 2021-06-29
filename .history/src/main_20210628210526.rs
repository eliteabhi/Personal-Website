#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;
use rocket::http::ContentType;
// use rocket::response::Redirect;

#[launch]
fn rocket() -> _ { // Launch website

    rocket::build()
                .attach(Template::fairing())
                .mount("/", routes![index, css])

}

// Structs/Contexts
#[derive(serde::Serialize)]
struct IndexContext {
    date: &'static str,
    name: &'static str,

    parent: &'static str
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

// CSS
#[get("/css/index.css")]
fn css() -> (ContentType, &'static str) {

    (ContentType::CSS, std::include_str!("../css/index.css"))

}
