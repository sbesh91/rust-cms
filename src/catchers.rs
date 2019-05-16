use rocket::response::content;

#[catch(404)]
pub fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>Sorry, but '{}' is not a valid path!</p>
            <a href=\"/\">Return Home</a>",
            req.uri()))
}