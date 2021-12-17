use rocket_dyn_templates::Template;
use rocket::http::ContentType;
use std::fs;

//Helper Functions
pub fn read_file(path: String) -> std::fs::File { return fs::File::open(path).expect("Failed to open file"); }

#[get("/css/pages/contactMe.css")]
pub fn contact_css() -> (ContentType, fs::File) { return (ContentType::CSS, read_file("css/pages/contact.css".to_string())); }

#[derive(serde::Serialize)]
pub struct ContactMeContext {
    
    name: &'static str,
    email: &'static str,
    subject: &'static str,
    message: &'static str,
    
    title: &'static str
    
}

#[get("/")]
pub fn contact() -> Template {
    
    Template::render("pages/contact", &ContactMeContext {
        
        name: "",
        email: "",
        subject: "",
        message: "",

        title: "Contact Me"
        
    })

}
