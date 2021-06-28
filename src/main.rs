#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::response::Redirect;

#[launch]
fn rocket() -> _ { //Launch website

    rocket::build().attach(Template::fairing()).mount("/", routes![index])

}

//Structs/Contexts

#[derive(serde::Serialize)]
struct IndexContext {

    date: &'static str,
    name: &'static str,

    parent: &'static str
}

//Index(Default) page
#[get("/")]
fn index() -> Template {

    Template::render("index", &IndexContext{

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        parent: "layout"
    })

}
