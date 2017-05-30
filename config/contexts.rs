use gerust_router::*;
use url::Url;
use gerust_http;
use gerust_context;

pub struct HttpContext {
    url: Url
}

impl gerust_context::Context for HttpContext {

}

impl gerust_http::HttpContext for HttpContext {
    type Router = Router;
    type Request = Params;

    fn router(&self) -> Self::Router {
        Router::new()
    }
    fn request(&self) -> Self::Request {
        Params::new()
    }

    fn url(&self) -> &Url {
        &self.url
    }
}

