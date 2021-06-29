use rocket_dyn_templates::Template;

#[derive(serde::Serialize)]
pub struct AboutContext {

    date: &'static str,
    name: &'static str,

    title: &'static str,
    parent: &'static str

}

#[get("/about")]
pub fn about() -> Template {

    Template::render("pages/about", &AboutContext {

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        title: "About",
        parent: "layout"

    })

}
