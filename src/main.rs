use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;

mod index;

#[derive(serde::Serialize)]
struct Message {
    user: &'static str,
    body: &'static str
}

#[get("/")] //Index of Website
fn ind() -> Template {
    Template::render("index", &BoardContext {
        current_user: Some("hi"),
        messages: vec![Message{user: "userA", body: "This is the first test message."},
                        Message{user: "userB", body: "This is the second test message."}],
        parent: "layout"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing()).mount("/", routes![ind])
}