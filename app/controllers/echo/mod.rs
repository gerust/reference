use gerust_controller::Controller;
use gerust_http::HttpContext;
// this needs to be replaced
use gerust_router::Params;

pub struct EchoController<C: HttpContext> {
    context: &'static C
}

impl<C: HttpContext> Controller<C> for EchoController<C> {
    type Params = Params;
    type Result = Params;

    fn new(context: &'static C) -> Self { EchoController { context } }

    fn execute(&self, input: Params) -> Params {
        let mut params = Params::new();
        params.insert("echo".into(), "echo".into());
        params
    }
}