use super::server::Handler;
use super::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("It works!".to_string()))
    }
}