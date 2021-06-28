use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;

mod index;

#[get("/")] //Index of Website
fn ind() -> Template {
    Template::render("layout", "hi")
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing())
}