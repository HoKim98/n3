use rocket::Request;

#[catch(404)]
pub fn view(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
