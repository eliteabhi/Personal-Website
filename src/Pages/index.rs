use rocket_dyn_templates::Template;

#[derive(serde::Serialize)]
pub struct IndexContext {

    date: &'static str,
    name: &'static str,

    title: &'static str,
    parent: &'static str

}

#[get("/")]
pub fn index() -> Template {

    Template::render("pages/index", &IndexContext {

        date: "1/21/2001",
        name: "Abhi Rangarajan",

        title: "Home",
        parent: "layout"
    })
    
}

