use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome to Rust Test HTTP Server</h1>".to_string())),
                "/say" => Response::new(StatusCode::Ok, Some("<h1>Welcome, my friend!</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}