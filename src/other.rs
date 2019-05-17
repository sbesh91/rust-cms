
#[get("/other")]
pub fn some() -> &'static str {
  "other..."
}

#[get("/blah")]
pub fn blah() -> &'static str {
  "blah"
}