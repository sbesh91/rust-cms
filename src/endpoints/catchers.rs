use rocket::response::content;

#[catch(404)]
pub fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>Sorry, but '{}' is not a valid path!</p>
            <a href=\"/\">Return Home</a>",
            req.uri()))
}

// todo: update this response
#[catch(400)]
pub fn bad_request(req: &rocket::Request) -> content::Html<String> {
  content::Html(format!("<p>Internal Server Error '{}' </p>", req.uri()))
}

#[catch(500)]
pub fn internal_server_error(req: &rocket::Request) -> content::Html<String> {
  content::Html(format!("<p>Internal Server Error '{}' </p>", req.uri()))
}